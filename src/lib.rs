//! `wordcount`は、シンプルな文字、単語または行の出現頻度を数える機能を提供する。
//! なお、行は行数ではなく、その行で記録されている文字列が一致する行の数を数える。
//! 詳しくは、[`count`](fn.count.html)関数のドキュメントを参照すること。
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// [`count`](fn.count.html)で使用するオプション
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    /// 文字の出現頻度を数える。
    Char,
    /// 単語の出現頻度を数える。
    Word,
    /// 行の出現頻度を数える。
    Line,
}

/// オプションのデフォルトは、[`word`](enum.CountOption.html#variant.Word)。
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// `input`から1行ずつUTF-8文字列を読み込み、出現頻度を数える。
///
/// 頻度を数える対象は、オプションによって制御される。
/// * [`CountOption::Char`](enum.CountOption.html#variant.Char): Unicodeの1文字ごと。
/// * [`CountOption::Word`](enum.CountOption.html#variant.Word): 正規表現`\w+`にマッチする単語ごと。
/// * [`CountOption::Line`](enum.CountOption.html#variant.Line): `\n`または`\r\n`で区切られた1行ごと。
///
/// # Panics
///
/// 入力がUTF-8文字列でない場合は、パニックを起こす。
/// 
/// # Examples
/// 
/// 入力中の単語の出現頻度を数える例。
/// 
/// ```
/// use std::io::Cursor;
/// use kuroyasu_bicycle_book_wordcount::{count, CountOption};
/// 
/// 
/// let mut input = Cursor::new("aa bb cc bb");
/// let freqs = count(input, CountOption::Word);
/// assert_eq!(freqs["aa"], 1);
/// assert_eq!(freqs["bb"], 2);
/// assert_eq!(freqs["cc"], 1);
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        match option {
            CountOption::Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            CountOption::Word => {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            CountOption::Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }

    freqs
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    #[test]
    fn wordcount_works() {
        use std::io::Cursor;
        let mut exp = HashMap::new();
        exp.insert("aa".to_string(), 1);
        exp.insert("bb".to_string(), 2);
        exp.insert("cc".to_string(), 1);

        assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
    }

    // #[test]
    // fn wordcount_fails() {
    //     use std::io::Cursor;
    //     let mut exp = HashMap::new();
    //     exp.insert("aa".to_string(), 1);
    //
    //     assert_eq!(count(Cursor::new("aa cc dd"), CountOption::Word), exp);
    // }

    #[test]
    fn wordcount_works2() {
        use std::io::Cursor;
        let mut exp = HashMap::new();
        exp.insert("aa".to_string(), 1);
        exp.insert("cc".to_string(), 1);
        exp.insert("dd".to_string(), 1);

        assert_eq!(count(Cursor::new("aa cc dd"), CountOption::Word), exp);
    }

    #[test]
    fn result_test() -> io::Result<()> {
        use std::fs::{read_to_string, remove_file, write};
        // ?演算子を使用する。
        write("test.txt", "message")?;
        let message = read_to_string("test.txt")?;
        remove_file("test.txt")?;
        assert_eq!(message, "message");

        Ok(())
    }

    // #[test]
    // fn result_err() -> io::Result<()> {
    //     use std::fs::remove_file;
    //     remove_file("no_such_file")
    // }

    #[test]
    #[should_panic]
    fn wordcount_do_not_contain_unknown_words() {
        use std::io::Cursor;
        count(
            Cursor::new([
                b'a',   // a
                0xf0, 0x90, 0x80,   // 出鱈目なバイト列
                0xe3, 0x81, 0x82,   // あ
            ]),
            CountOption::Word,
        );
    }

    #[test]
    #[ignore]
    fn large_test() {
        println!("large test");
    }
}
