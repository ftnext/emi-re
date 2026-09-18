use regex::Regex;
use std::sync::LazyLock;

static PATTERNS: LazyLock<[Regex; 3]> = LazyLock::new(|| {
    let basic_latin = r"\u0000-\u007f";
    let blocks = concat!(
        r"\u4e00-\u9fff", // CJK UNIFIED IDEOGRAPHS
        r"\u3040-\u309f", // HIRAGANA
        r"\u30a0-\u30ff", // KATAKANA
        r"\u3000-\u303f", // CJK SYMBOLS AND PUNCTUATION
        r"\uff00-\uffef", // HALFWIDTH AND FULLWIDTH FORMS
    );
    [
        format!("([{blocks}]) ([{blocks}])"),
        format!("([{blocks}]) ([{basic_latin}])"),
        format!("([{basic_latin}]) ([{blocks}])"),
    ]
    .map(|pattern| Regex::new(&pattern).unwrap())
});

pub fn remove_spaces(text: &str) -> String {
    let mut text = text.to_owned();
    for pattern in PATTERNS.iter() {
        while pattern.is_match(&text) {
            text = pattern.replace_all(&text, "${1}${2}").into_owned();
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::remove_spaces;

    #[test]
    fn removes_spaces_around_japanese() {
        for (input, expected) in [
            ("アルゴリズム C", "アルゴリズムC"),
            ("アルゴ B リズム C", "アルゴBリズムC"),
            ("アイ の 歌声 を 聴か せ て", "アイの歌声を聴かせて"),
            (
                "ういっす ういっす ういっすー✌️",
                "ういっすういっすういっすー✌️",
            ),
            (
                "検索 エンジン 自作 入門 を 買い ました ！！！",
                "検索エンジン自作入門を買いました！！！",
            ),
        ] {
            assert_eq!(remove_spaces(input), expected);
        }
    }

    #[test]
    fn preserves_other_whitespace() {
        for text in ["Algorithm C", "Coding the Matrix"] {
            assert_eq!(remove_spaces(text), text);
        }
    }
}
