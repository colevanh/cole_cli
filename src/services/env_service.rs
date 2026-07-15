use std::env::{self};

pub fn print_vars() {
    for (key,value) in env::vars_os() {
        println!("{:?}: {:?}", key, value);
    }
}

