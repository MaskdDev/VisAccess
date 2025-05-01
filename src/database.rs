use crate::structs::user::User;
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use serenity::prelude::TypeMapKey;

// The database struct for VisAccess
#[derive(Clone)]
pub struct Database {
    pub connection: MultiplexedConnection,
}

// Implement TypeMapKey to allow for storage in serenity client data
impl TypeMapKey for Database {
    type Value = Database;
}

impl Database {
    /// Get a database struct from a Redis connection arc.
    pub fn from_connection(connection: MultiplexedConnection) -> Self {
        Self { connection }
    }

    /// Get a new cloned connection for the database.
    pub fn get_connection(&self) -> MultiplexedConnection {
        self.connection.clone()
    }

    /// Get a user from the database.
    pub async fn get_user(&self, user_id: u64) -> User {
        if self
            .get_connection()
            .exists(user_id)
            .await
            .expect("Error checking for user existence.")
        {
            // Get user
            let user: User = self
                .get_connection()
                .get(user_id)
                .await
                .expect("Error getting user.");

            // Return loaded user
            user.load(user_id, self)
        } else {
            // Create new user
            let user = User::new(user_id, self);

            // Insert new user
            let _: () = self
                .get_connection()
                .set(user_id, &user)
                .await
                .expect("Error setting user for creation.");

            // Return new user
            user
        }
    }

    /// Update a user in the database
    pub async fn update_user(&self, user_id: u64, user: &User) {
        // Set user
        let _: () = self
            .get_connection()
            .set(user_id, user)
            .await
            .expect("Error setting user for an update.");
    }
}
