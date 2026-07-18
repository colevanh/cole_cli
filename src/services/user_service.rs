use rand::prelude::*;
use crate::models::testuser::TestUser;

const FIRST_NAMES: [&str;5] = [
    "tamra",
    "tommy",
    "reba",
    "tina",
    "mark"
];

const LAST_NAMES: [&str;5] = [
    "smith",
    "mcdonald",
    "lawson",
    "ness",
    "vanhoogenstyn"
];

pub fn create_email(first_name: &str, last_name: &str) -> String {
    let mut rng: ThreadRng = rand::rng();
    let random_number = rng.random_range::<u32, _>(100..=999).to_string();

    format!("{first_name}.{last_name}{random_number}@gmail.com")

}

pub fn build_test_user() -> TestUser {
    let mut rng = rand::rng();

    let mut first_name = FIRST_NAMES[rng.random_range(0..=4)].to_string();
    let mut last_name = LAST_NAMES[rng.random_range(0..=4)].to_string();
    let random_age = rng.random_range::<u32, _>(15..75);
    let email = create_email(&first_name, &last_name);
    
    TestUser {
        first_name: first_name.clone(),
        last_name: last_name.clone(),
        email,
        age: random_age,
        uuid: rng.random::<u128>(),
    }
}

