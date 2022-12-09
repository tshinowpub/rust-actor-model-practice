use crate::extractor::option_detector::OptionDetector;

pub struct ArgumentExtractor {}

impl ArgumentExtractor {
    pub fn extract(arguments: Vec<String>, execute_path: &String) -> Vec<String> {
        let user_arguments: Vec<String> = arguments
            .iter()
            .filter_map(|s| {
                return match s {
                    s if (s != execute_path && !OptionDetector::is_option(s)) => s.parse::<String>().ok(),
                    _ => None
                }
            })
            .collect();

        user_arguments
    }
}
