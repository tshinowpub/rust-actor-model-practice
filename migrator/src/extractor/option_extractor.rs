pub struct OptionExtractor {}

use crate::extractor::option_detector::OptionDetector;

impl OptionExtractor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract(&self, arguments: Vec<String>, execute_path: &String) -> Vec<String> {
        return arguments
            .iter()
            .filter_map(|s| {
                return match s {
                    s if (s != execute_path && OptionDetector::is_option(s)) => s.parse::<String>().ok(),
                    _ => None
                }
            })
            .collect();
    }
}
