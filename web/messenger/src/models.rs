use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::schema::messages;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    id: i32,
    username: String
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Message {
    id: i32,
    author_id: i32,
    recipient_id: i32,
    text: String,
    date: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="messages"]
pub struct NewMessage {
    pub author_id: i32,
    pub recipient_id: i32,
    pub text: String,
    pub date: DateTime<Utc>
}

