use clap::ValueEnum;

#[derive(PartialEq, Debug, Clone, ValueEnum)]
pub enum MigrateType {
    Up,
    Down,
}

impl MigrateType {
    pub fn is_up(&self) -> bool {
        *self == MigrateType::Up
    }

    pub fn is_down(&self) -> bool {
        *self == MigrateType::Down
    }
}
