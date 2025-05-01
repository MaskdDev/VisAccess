use crate::types::PoiseCommand;

// Get command modules
mod status;

// Create command loader function
pub fn get_commands() -> Vec<PoiseCommand> {
    return vec![status::status()];
}
