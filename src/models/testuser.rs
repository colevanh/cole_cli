use std::fmt;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::error::Error;
use serde_json::Result as SJResult;
use uuid::Uuid;

// TODO
// fields to add: username, address, created_at, is_active
// * structs should use the String owned type rather than the &str slice type
// * we want each instance of this struct to own all its data.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TestUser {
    pub first_name: String,             // First name (required)
    pub last_name: String,             // Last name (required)
    pub email: String,                 // User email (required)
    pub role: UserRole,                // UserRole value (required)
    pub age: Option<u32>,              // User age (optional)
    pub phone_number: Option<String>,  // User phone number (optional)
    pub uuid: Option<u128>,            // User uuid (optional)
}

impl TestUser {
    
    /// Get user's full name
    /// Concats and returns first + last name
    pub fn get_full_name(&self) -> String {
        let first_name_copy = &self.first_name;
        let last_name_copy = &self.last_name;
        format!("{first_name_copy} {last_name_copy}")
    }
}

/// Builder struct used to create TestUser objects
pub struct TestUserBuilder {
    /// * See TestUser field comments for info
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: UserRole,
    pub age: Option<u32>,
    pub phone_number: Option<String>,   
    pub uuid: Option<u128>,
}

impl TestUserBuilder {
    // Builder constructor - only mandatory fields required
    pub fn new(first_name: &str, last_name: &str, email: &str) -> Self {
        TestUserBuilder { 
            first_name: first_name.to_string(), // to_string() for ownership
            last_name: last_name.to_string(), // to_string() for ownership
            email: email.to_string(),         // to_string() for ownership
            role: UserRole::Standard,         // default UserRole type
            age: None, 
            phone_number: None, 
            uuid: None 
        }
    }

    pub fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }

    pub fn phone_number(mut self, phone_number: &str) -> Self {
        self.phone_number = Some(phone_number.to_string());
        self
    }

    pub fn uuid(mut self, uuid: u128) -> Self {
        self.uuid = Some(uuid);
        self
    }

    pub fn build(self) -> TestUser {
        TestUser {
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            role: self.role,
            age: self.age,
            phone_number: self.phone_number,
            uuid: self.uuid
        }
    }
}

/// PhoneNumber struct that models a field for a TestUser
/// * eventually implement this for TestUser. A String is fine for noww
#[derive(Debug, Clone)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(number: &str) -> Result<Self, String> {
        if Self::is_valid(number) {
            Ok(PhoneNumber(number.to_string()))
        } else {
            Err(format!("Invalid phone number format: '{}'. Expected ###-###-####", number))
        }
    }

    fn is_valid(number: &str) -> bool {
        let parts: Vec<&str> = number.split('-').collect();

        if parts.len() != 3 {
            return false;
        }

        parts[0].len() == 3 && parts[0].chars().all(|c| c.is_ascii_digit()) &&
        parts[1].len() == 3 && parts[1].chars().all(|c| c.is_ascii_digit()) &&
        parts[2].len() == 4 && parts[2].chars().all(|c| c.is_ascii_digit())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
    
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Standard,
    Guest
}

impl Default for UserRole {
    fn default() -> Self { UserRole::Standard}
}

impl fmt::Display for TestUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "Name: {} {}\nAge: {}\nEmail: {}\nPhone: {:?}\nuuid: {}\n",
            self.first_name, self.last_name, self.age.unwrap(), self.email, &self.phone_number, self.uuid.unwrap()
        )
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::assert;

use super::*;

    const FIRST_NAMES: [&str;3] = ["Mark", "Stephanie", "Reba"];
    const LAST_NAMES: [&str;3] = ["Smith", "Pinkbottom", "VanHoogenstyn"];
    const TEST_EMAIL: &str = "test_email@gmail.com";

    fn build_mark_smith() -> TestUser {
        let new_user: TestUser = TestUserBuilder
            ::new(FIRST_NAMES[0], LAST_NAMES[0], TEST_EMAIL)
            .build();

        new_user
    }

    fn build_steph_pink() -> TestUser {
        let new_user: TestUser = TestUserBuilder
            ::new(FIRST_NAMES[1], LAST_NAMES[1], TEST_EMAIL)
            .build();

        new_user
    }

    fn build_reba_van() -> TestUser {
        let new_user: TestUser = TestUserBuilder
            ::new(FIRST_NAMES[2], LAST_NAMES[2], TEST_EMAIL)
            .build();

        new_user
    }

    /// Tests default TestUser construction via TestUserBuilder
    #[test]
    fn test_user_builder_no_optional_fields() {
        let first_name = FIRST_NAMES[0].to_string();
        let last_name: String = LAST_NAMES[0].to_string();
        let new_user: TestUser = TestUserBuilder
            ::new(&first_name, &last_name, "test_email@gmail.com")
            .build();

        assert_eq!(None, new_user.age);
        assert_eq!(None, new_user.uuid);
        assert_eq!(None, new_user.phone_number);
    }

    /// Tests TestUser's age via TestUserBuilder usage
    #[test]
    fn test_user_builder_with_age() {
        let first_name: String = FIRST_NAMES[1].to_string();
        let last_name: String = LAST_NAMES[1].to_string();
        let new_user: TestUser = TestUserBuilder
            ::new(&first_name, &last_name, "test_email&gmail.com")
            .age(35)
            .build();

        assert_eq!(35, new_user.age.unwrap());
    }

    #[test]
    fn test_full_name() {
        let test_user_1 = build_mark_smith();
        let test_user_2 = build_steph_pink();
        let test_user_3: TestUser = build_reba_van();

        assert_eq!("Mark Smith", test_user_1.get_full_name());
        assert_eq!("Stephanie Pinkbottom", test_user_2.get_full_name());
        assert_eq!("Reba VanHoogenstyn", test_user_3.get_full_name());
    }

}