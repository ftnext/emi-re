use regex::Regex;

pub fn to_snake_case(camel_case_str: &str) -> String {
    let re1 = Regex::new(r"(.)([A-Z][a-z]+)").unwrap();
    let s1 = re1.replace_all(camel_case_str, "${1}_${2}");
    let re2 = Regex::new(r"([a-z0-9])([A-Z])").unwrap();
    re2.replace_all(&s1, "${1}_${2}").to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_word() {
        assert_eq!(to_snake_case("port"), "port");
    }

    #[test]
    fn multiple_words() {
        assert_eq!(to_snake_case("gatherUsageStats"), "gather_usage_stats");
        assert_eq!(
            to_snake_case("enforceSerializableSessionState"),
            "enforce_serializable_session_state"
        );
        assert_eq!(
            to_snake_case("showPyplotGlobalUse"),
            "show_pyplot_global_use"
        );
    }
}
