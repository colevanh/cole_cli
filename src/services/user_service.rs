use rand::prelude::*;
use fake::{Fake, faker::name::raw::*, faker::internet::en::FreeEmailProvider, faker::number::en::Digit, faker::phone_number::en::CellNumber};
use fake::locales::*;
use serde_json;

use crate::models::testuser::{TestUser, UserRole};

pub fn create_email(first_name: &str, last_name: &str) -> String {
    let mut rng: ThreadRng = rand::rng();
    let random_number = rng.random_range::<u32, _>(100..=999).to_string();
    let fake_provider: String = FreeEmailProvider().fake();

    format!("{first_name}.{last_name}{random_number}{fake_provider}")

}

pub fn build_test_users(count: u32) -> Vec<TestUser> {
    let mut user_vec: Vec<TestUser> = Vec::new();
    let mut rng = rand::rng();

    for _ in 0..count {
        let first_name: String = FirstName(EN).fake();
        let last_name: String = LastName(EN).fake();
        let email: String = create_email(&first_name, &last_name);

        let new_user: TestUser = TestUser {
            first_name,
            last_name,
            email,
            phone_number: CellNumber().fake(),
            age: Some(rng.random_range::<u32, _>(15..=75)),
            uuid: Some(rng.random::<u128>()),
            role: UserRole::Standard
        };
        user_vec.push(new_user);
    };

    user_vec
}

pub fn generate_first_names(count: u32) -> Vec<String> {

    let mut fake_first_names = Vec::new();
    for _ in 0..count {
        let fake_first_name: String = FirstName(EN).fake();
        fake_first_names.push(fake_first_name);
    }

    fake_first_names
}

pub fn generate_last_names(count: u32) -> Vec<String> {

    let mut fake_last_names = Vec::new();
    for _ in 0..count {
        let fake_first_name: String = LastName(EN).fake();
        fake_last_names.push(fake_first_name);
    }

    fake_last_names
}

pub fn serialize_user(user: &TestUser) -> Result<String, serde_json::Error> {
    serde_json::to_string(user)
}

