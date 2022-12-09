pub struct OptionExtractor {}

impl OptionExtractor {
    pub fn extract(arguments: Vec<String>, execute_path: &String) -> Vec<String> {
        return arguments
            .iter()
            .filter_map(|s| {
                return match s {
                    s if (s != execute_path && Self::is_option(s)) => s.parse::<String>().ok(),
                    _ => None
                }
            })
            .collect();
    }

    pub fn is_option(pattern: &String) -> bool {
        return match pattern.find("--") {
            Some(found) if found == 0 => true,
            _ => false
        }
    }
}
