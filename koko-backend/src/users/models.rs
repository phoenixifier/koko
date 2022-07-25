#![allow(clippy::extra_unused_lifetimes)]

use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub lang: String,
    pub admin: bool,
}
