use anyhow::Context;
use serde::Deserialize;

pub(crate) struct Parser {
}

impl Parser {
    pub fn from_json_file<T: for<'a> Deserialize<'a>>(file: &std::fs::File) -> anyhow::Result<T> {
        let result: T = serde_json::from_reader(file)
            .context(format!("Cannot parse json file. File name: {:?}", file))?;

        Ok(result)
    }
}
