use std::collections::HashMap;

#[derive(Default)]
pub struct OptionLexer {}

const OPTION_PREFIX: &str = "--";

impl OptionLexer {
    pub fn parse(&self, args: &Vec<String>) -> HashMap<String, Option<String>> {
        let binding = args.to_vec();
        let iter = binding.iter();

        iter
            .map(|s| (s.to_string().split_off(OPTION_PREFIX.len()), None))
            .collect::<HashMap<String, Option<String>>>()
    }
}
