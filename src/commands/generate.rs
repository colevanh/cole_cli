use crate::services::user_service::build_test_user;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let my_user = build_test_user();
    println!("{my_user}");
    Ok(())
}