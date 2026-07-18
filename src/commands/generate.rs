use crate::services::user_service::{build_test_users, serialize_user};

pub fn run(count: u32) -> Result<(), Box<dyn std::error::Error>> {
    let new_users = build_test_users(count);

    for user in new_users {
        let json_string = serialize_user(&user);
        println!("{}\n", json_string)
    }
    Ok(())
}