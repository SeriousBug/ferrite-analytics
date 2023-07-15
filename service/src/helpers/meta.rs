//! The `Meta` table/entity is used as a key-value store to store arbitrary
//! data, such as the JWT secret key.
//!
//! This file serves as the list of keys that are used in the Meta table. If you
//! are adding a new key to the Meta table, add it here. If you are removing
//! something, don't remove it from this file so it doesn't get reused in the
//! future (for backwards compatibility).

pub const JWT_SECRET_KEY: &'static str = "JWT Secret";
