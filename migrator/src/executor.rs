#[derive(Default)]
pub struct Executor {}

impl Executor {
    pub fn execute(command: &String) {
        println!("{}", command);
    }
}
