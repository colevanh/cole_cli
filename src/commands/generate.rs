use crate::services::user_service::{build_test_users, serialize_user};

pub fn run(count: u32) -> Result<(), Box<dyn std::error::Error>> {
    let new_users = build_test_users(count);

    for user in new_users {
        let json_result = serialize_user(&user);
        let json_string = match json_result {
            Ok(string) => string,
            Err(err) => panic!("bad serialization"),
        };
        println!("{}\n", json_string)
    }
    Ok(())
}