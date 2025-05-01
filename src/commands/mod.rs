use crate::types::PoiseCommand;

// Load command modules
mod moderation;

// Create command loader function
pub fn get_commands() -> Vec<PoiseCommand> {
    // Initialise commands
    let mut loaded_commands: Vec<PoiseCommand> = vec![];

    // Add category commands
    loaded_commands.append(&mut moderation::get_commands());

    // Return loaded commands
    return loaded_commands;
}
