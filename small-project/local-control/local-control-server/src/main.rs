#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

use rocket::{
    fs::FileServer,
    serde::{
        json::{serde_json::json, Value},
        Serialize,
    },
};

use rusqlite::Connection;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Order {
    id: i32,
    name: String,
    path: String,
}

lazy_static! {
    static ref ORDER_ARR: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref CONN: Mutex<Connection> =
        Mutex::new(Connection::open("D:\\tauri\\local_control.db").unwrap());
}

use screenshots::Screen;
use std::fs;

#[get("/get_img")]
fn get_img() {
    let screens = Screen::all().unwrap();

    for screen in screens {
        println!("capturer {:?}", screen);
        let image = screen.capture().unwrap();
        let buffer = image.buffer();
        fs::write("static/screen.png", &buffer).unwrap();

    }
}

#[get("/get_task")]
fn get_task() -> Value {
    let conn = CONN.lock().unwrap();

    let mut stmt = conn
        .prepare("SELECT id, name, path FROM order_record")
        .unwrap();
    let mut rows = stmt.query([]).unwrap();

    let mut list: Vec<Order> = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let order = Order {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            path: row.get(2).unwrap(),
        };

        list.push(order);
    }

    json!(list)
}

#[get("/add_task/<uid>?<id>")]
fn add_task(uid: Option<&str>, id: Option<usize>) -> Value {
    let mut order = Order {
        id: Default::default(),
        name: Default::default(),
        path: Default::default(),
    };

    println!("{}", uid.unwrap());

    if uid.unwrap().eq("c0dcf2ce03e8f725ad35fe2fefef005b") {
        let conn = CONN.lock().unwrap();

        let mut stmt = conn
            .prepare("SELECT id, name, path FROM order_record WHERE id = ?")
            .unwrap();

        let mut rows = stmt.query([id.unwrap()]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            order.id = row.get(0).unwrap();
            order.name = row.get(1).unwrap();
            order.path = row.get(2).unwrap();
        }

        ORDER_ARR.lock().unwrap().push(order.path);
    }

    json!({})
}

#[get("/get_task_path")]
fn get_task_path() -> Value {
    let path;
    match ORDER_ARR.lock().unwrap().get(0) {
        Some(p) => path = p.clone(),
        None => {
            println!("no string");
            return json!({});
        }
    }

    ORDER_ARR.lock().unwrap().remove(0);
    json!({"path": path.to_string()})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/public", FileServer::from("static/"))
        .mount("/", routes![get_task, add_task, get_task_path, get_img])
}
