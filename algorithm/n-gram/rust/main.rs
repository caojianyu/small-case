fn main() {
    let text1 = "幽灵轰炸机";
    // let text2 = "时间简史的作者";
    let word_group = n_gram(text1);
    for c in word_group {
        println!("{}", c);
    }
}

fn n_gram(text: &str) -> Vec<String> {
    let mut word_group = Vec::new();
    let text_len = text.chars().count();
    let mut i = 2;
    while i < 4 {
        let mut j = 0;
        while j < text_len {
            let end_index = i + j;
            if end_index <= text_len {
                let mut word = String::new();
                for (index, c) in text.chars().enumerate() {
                    if index >= j && index < end_index {
                        word.push(c);
                    }
                }
                word_group.push(word);
            }
            j += 1;
        }
        i += 1;
    }
    word_group
}
