use rand::prelude::*;

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
    "van hoogenstyn"
];

pub fn build_test_user() -> TestUser {
    let mut rng = rand::rng();

    let first_name = FIRST_NAMES[rng.random_range(0..=4)];
    let last_name = LAST_NAMES[rng.random_range(0..=4)];
    let random_age = rng.random_range::<u32, _>(15..75);
    
    TestUser {
        name: first_name.to_string(),
        email: "test".to_string(),
        age: random_age
    }
}