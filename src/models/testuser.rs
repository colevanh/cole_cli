use std::fmt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use serde_json::Result;

// TODO
// fields to add: phone_number, username, address, created_at, is_active
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TestUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: u32,

    pub uuid: u128,

}


pub enum UserRole {
    Admin,
    Standard,
    Guest
}



impl fmt::Display for TestUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "Name: {} {}\nEmail: {}\nAge: {}\nuuid: {}",
            self.first_name, self.last_name, self.email, self.age, self.uuid
        )
    }
}