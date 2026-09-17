use regex::Regex;

pub fn extract_json(markdown: &str) -> String {
    let re = Regex::new(r"(?s)```json\s*(.*?)\s*```").unwrap();
    let captures = re.captures(markdown).unwrap();
    captures.get(1).unwrap().as_str().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json() {
        let markdown = r#"
Here is some text.
```json
{
  "key": "value",
  "number": 123
}
```
More text here.
"#;
        assert_eq!(
            extract_json(markdown),
            r#"{
  "key": "value",
  "number": 123
}"#
        )
    }
}
