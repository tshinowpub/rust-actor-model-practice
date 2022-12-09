pub struct OptionDetector {
}

impl OptionDetector {
    pub(crate) fn is_option(pattern: &String) -> bool {
        return match pattern.find("--") {
            Some(found) if found == 0 => true,
            _ => false
        }
    }
}
