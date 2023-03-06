use anyhow::{Context, Result};
use std::path::Path;

#[derive(PartialEq, Debug, Clone)]
pub enum MigrateOperationType {
    CreateTable,
    DeleteTable,
    UndefinedOperation(String),
}

impl MigrateOperationType {
    pub fn resolve(name: &Path) -> Result<MigrateOperationType> {
        if name
            .to_str()
            .context("Failed to_str name.")?
            .contains(".create_table.")
        {
            return Ok(MigrateOperationType::CreateTable);
        }

        if name
            .to_str()
            .context("Failed to_str name.")?
            .contains(".delete_table.")
        {
            return Ok(MigrateOperationType::DeleteTable);
        }

        Ok(MigrateOperationType::UndefinedOperation(
            name.to_str().context("Failed to_str name.")?.to_string(),
        ))
    }
}
