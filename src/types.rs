// The client data struct - not a type, but it can go here :)
#[derive(Debug)]
pub struct ClientData {}

// Error and context types used by all command functions
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, ClientData, Error>;

/// Poise command type
pub type PoiseCommand = poise::Command<ClientData, Error>;
