use std::env;

pub fn is_development() -> bool {
    match env::var("RUST_ENV") {
        Ok(value) => value == "development",
        _ => false,
    }
}

pub fn is_production() -> bool {
    match env::var("RUST_ENV") {
        Ok(value) => value == "production",
        _ => false,
    }
}

#[test]
fn development_is_set() {
    env::set_var("RUST_ENV", "development");
    assert!(is_development());
}

#[test]
fn production_is_set() {
    env::set_var("RUST_ENV", "production");
    assert!(is_production());
    assert!(!is_development());
}

#[test]
fn nothing_is_set() {
    env::remove_var("RUST_ENV");
    assert!(!is_production());
    assert!(!is_development());
}
