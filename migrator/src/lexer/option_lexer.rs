use std::collections::HashMap;

#[derive(Default)]
pub struct OptionLexer {}

const OPTION_PREFIX: &str = "--";

pub(crate) type Options = HashMap<String, Option<String>>;

impl OptionLexer {
    pub fn parse(&self, args: &Vec<String>) -> Options {
        let binding = args.to_vec();
        let iter = binding.iter();

        iter
            .map(|s| (
                s.to_string().split_off(OPTION_PREFIX.len()), None)
            )
            .collect::<Options>()
    }
}
