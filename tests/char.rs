use std::io::Cursor;

use bycycle_book_wordcount::{count, CountOption};

#[macro_use]
mod utils;

#[test]
fn charcount_works() {
    let input = Cursor::new(b"abadracadabra");
    let freqs = count(input, CountOption::Char);
    assert_map!(freqs,
        {
            "a" => 6,
            "b" => 2,
            "c" => 1,
            "d" => 2,
            "r" => 2
        }
    );
}

#[test]
fn charcount_utf8() {
    let input = Cursor::new(
        r#"
天地玄黃
宇宙洪荒
日月盈昃
辰宿列張
"#,
    );
    let freqs = count(input, CountOption::Char);
    assert_eq!(freqs.len(), 16);
    for (_, count) in freqs {
        assert_eq!(count, 1);
    }
}