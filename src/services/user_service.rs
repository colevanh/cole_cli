use rand::prelude::*;
use fake::{Fake, faker::name::raw::*, faker::internet::en::FreeEmailProvider, faker::number::en::Digit};
use fake::locales::*;
use serde_json::json;

use crate::models::testuser::TestUser;

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
        let mut first_name: String = FirstName(EN).fake();
        let mut last_name: String = LastName(EN).fake();
        let email: String = create_email(&first_name, &last_name);

        let new_user: TestUser = TestUser {
            first_name: first_name,
            last_name: last_name,
            email,
            age: rng.random_range::<u32, _>(15..75),
            uuid: rng.random::<u128>()
        };
        user_vec.push(new_user);
    };

    user_vec
}

pub fn serialize_user(user: &TestUser) -> String {
    serde_json::to_string(user).unwrap()
}