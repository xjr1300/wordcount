use std::io::Cursor;

use wordcount::{count, CountOption};

#[macro_use]
mod utils;

#[test]
fn linecount_works() {
    let input = Cursor::new(
        r#"Tokyo, Japan
Kyoto, Japan
Tokyo, Japan
Shanghai, China
"#
    );
    let freqs = count(input, CountOption::Line);
    assert_map!(freqs, {
        "Tokyo, Japan" => 2,
        "Kyoto, Japan" => 1,
        "Shanghai, China" => 1
    });
}

#[test]
fn linecount_lfcr() {
    let input = Cursor::new("aa\r\nbb\r\ncc\r\nbb");
    let freqs = count(input, CountOption::Line);
    assert_map!(freqs, {
        "aa" => 1,
        "bb" => 2,
        "cc" => 1
    });
}