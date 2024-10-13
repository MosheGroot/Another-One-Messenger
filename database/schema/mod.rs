pub struct User {
    pub id: i32,
    pub username: String,
}

pub struct Message {
    pub id: i32,
    pub sender_id: i32,
    pub content: String,
}
