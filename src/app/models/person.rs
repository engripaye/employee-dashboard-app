use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, Eq, Clone)]
pub struct Person {

    pub uuid: String,

    }