use std::{
    env,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    sync::Mutex,
    thread,
    time::Duration,
};

use rdev::{listen, simulate, Button, Event, EventType};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INSTRUCT: Mutex<String> = Mutex::new(String::new());
    static ref FILE_NAME: Mutex<String> = Mutex::new(String::new());
}

/// 根据帧数计算间隔时间
const INTERVAL: Duration = Duration::from_millis(1000 / 100);

fn main() {
    // 读取指令参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print!("指令错误！");
        return;
    }

    // 读取指令
    let mut instruct = INSTRUCT.lock().unwrap();
    instruct.push_str(&args[1]);

    // 记录值文件
    FILE_NAME.lock().unwrap().push_str(&args[2]);

    // 匹配指令
    match instruct.as_str() {
        "record" => {
            // 监听
            if let Err(error) = listen(callback) {
                println!("Error: {:?}", error)
            }
        }
        "run" => {
            let mut i = 0;
            // 注意！！！千万不要设置过多执行次数或长时间记录值，否则不容易夺回鼠标的控制权。
            while i < 1 {
                run_record();
                i += 1;
            }
        }
        _ => println!("指令错误！"),
    }
}

/// 监听回调
fn callback(event: Event) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(FILE_NAME.lock().unwrap().as_str())
        .unwrap();

    match event.event_type {
        EventType::MouseMove { x, y } => record_move(x.to_string(), y.to_string()),
        EventType::ButtonPress(button) => record_press(button, file),
        EventType::ButtonRelease(button) => record_release(button, file),
        _ => {}
    }
}

/// 记录用户鼠标移动
fn record_move(x: String, y: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(FILE_NAME.lock().unwrap().as_str())
        .unwrap();

    let mut val = String::new();
    val.push_str("move");
    val.push(',');
    val.push_str(&x);
    val.push(',');
    val.push_str(&y);
    val.push('\n');

    file.write_all(val.as_bytes()).unwrap();
    println!("指针记录值: x {},y {}", x, y);
}

/// 记录用户鼠标按下
fn record_press(button: Button, mut file: File) {
    let mut val = String::new();
    match button {
        Button::Left => val.push_str("press,Left\n"),
        Button::Right => val.push_str("press,Right\n"),
        Button::Middle => val.push_str("press,Middle\n"),
        _ => {}
    }
    file.write_all(val.as_bytes()).unwrap();
}

/// 记录用户鼠标弹起
fn record_release(button: Button, mut file: File) {
    let mut val = String::new();
    match button {
        Button::Left => val.push_str("release,Left\n"),
        Button::Right => val.push_str("release,Right\n"),
        Button::Middle => val.push_str("release,Middle\n"),
        _ => {}
    }
    file.write_all(val.as_bytes()).unwrap();
}

/// 复现用户操作
fn run_record() {
    let file = File::open(FILE_NAME.lock().unwrap().as_str()).expect("指令不存在！");
    let br = BufReader::new(file);
    for line in br.lines() {
        let instruct = line.unwrap();
        let arr: Vec<&str> = instruct.split(",").collect();
        let v0_sign = arr[0];
        let v1 = arr[1];
        match v0_sign {
            "move" => {
                let x = v1.parse::<f64>().unwrap();
                let y = arr[2].parse::<f64>().unwrap();
                send(&EventType::MouseMove { x, y })
            }
            "press" => {
                if v1.eq("Left") {
                    send(&EventType::ButtonPress(Button::Left))
                } else {
                    send(&EventType::ButtonPress(Button::Right))
                }
            }
            "release" => {
                if v1.eq("Left") {
                    send(&EventType::ButtonRelease(Button::Left))
                } else {
                    send(&EventType::ButtonRelease(Button::Right))
                }
            }
            _ => {}
        }
    }
}

fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    thread::sleep(INTERVAL);
}
