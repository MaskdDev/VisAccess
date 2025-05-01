use crate::helpers::time;
use crate::Database;
use poise::serenity_prelude as serenity;
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

/// The VisAccess user struct, to keep track of warns and blocks.
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct User {
    /// The ID of the user - not serialized.
    #[serde(skip)]
    pub id: u64,

    /// A vector containing all the warns for the user.
    pub warns: Vec<Warn>,

    /// The current number of active warns for the user.
    pub active_warns: i32,

    /// Whether or not the user is blocked from sending images.
    blocked: bool,

    /// When the user is no longer blocked from sending images.
    blocked_till: Option<u64>,

    /// The ID of the moderator who blocked the user, if any.
    blocked_by: Option<u64>,

    /// The database - not serialized.
    #[serde(skip)]
    pub database: Option<Database>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            warns: Vec::new(),
            active_warns: 0,
            blocked: false,
            blocked_till: None,
            blocked_by: None,
            database: None,
        }
    }
}

impl User {
    /// Create a new user.
    pub fn new(user_id: u64, database: &Database) -> Self {
        // Create a new user
        let mut user = Self::default();

        // Set user ID and database
        user.id = user_id;
        user.database = Some(database.clone());

        // Return new user
        user
    }

    /// Load in the user ID and database for this user, consuming the provided user and returning the updated user.
    pub fn load(mut self, user_id: u64, database: &Database) -> User {
        // Set user ID and database
        self.id = user_id;
        self.database = Some(database.clone());

        // Return user
        self
    }

    /// Get the database for the user.
    pub fn get_database(&self) -> Database {
        self.database
            .clone()
            .expect("No database present during update call.")
    }

    /// Update the user in the database.
    pub async fn update(&self) {
        // Update user
        self.get_database().update_user(self.id, self).await;
    }

    /// Warn the user.
    pub async fn warn(&mut self, moderator: serenity::UserId, message: serenity::Message) {
        // Add warn to warns
        self.warns
            .push(Warn::new(moderator.get(), message.id.get()));

        // Increment active warns
        self.active_warns += 1;

        // Update user
        self.update().await;
    }

    /// Block the user indefinitely.
    pub async fn block(&mut self, moderator: serenity::UserId) {
        // Block user
        self.blocked = true;
        self.blocked_till = None;
        self.blocked_by = Some(moderator.get());

        // Update user
        self.update().await;
    }

    /// Block the user up until a certain unix timestamo.
    pub async fn block_until(&mut self, moderator: serenity::UserId, until: u64) {
        // Block user
        self.blocked = true;
        self.blocked_till = Some(until);
        self.blocked_by = Some(moderator.get());

        // Update user
        self.update().await;
    }

    /// Unblock the user.
    pub async fn unblock(&mut self) {
        // Unblock user
        self.blocked = false;
        self.blocked_till = None;
        self.blocked_by = None;

        // Update user
        self.update().await;
    }

    /// Check if the user can send images.
    pub async fn blocked(&mut self) -> bool {
        // Check if user is blocked
        if !self.blocked {
            // Return false as the user is not blocked
            return false;
        } else if let Some(until) = self.blocked_till {
            if until <= time::now() {
                // Unblock user
                self.unblock().await;

                // Return false as the user is no longer blocked
                return false;
            }
        }

        // Return true as the user is blocked
        true
    }
}

/// The VisAccess warn struct, representing a single warn.
#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Clone)]
pub struct Warn {
    /// The ID of the moderator responsible for the warn.
    moderator: u64,

    /// The ID of the message relating to the warn.
    message: u64,
}

impl Warn {
    /// Create a new warn using a moderator's user ID and message's ID.
    pub fn new(moderator: u64, message: u64) -> Self {
        Self { moderator, message }
    }
}
