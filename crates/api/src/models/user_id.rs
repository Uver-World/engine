use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct UserId(pub String);
