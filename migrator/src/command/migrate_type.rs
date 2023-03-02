use clap::ValueEnum;

#[derive(PartialEq, Debug, Clone, ValueEnum)]
pub enum MigrateType {
    Up,
    Down,
}
