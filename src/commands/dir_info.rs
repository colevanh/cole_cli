use crate::services::env_service::print_vars;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    print_vars();
    Ok(())
}