#[cfg(test)]
pub mod fixtures;
#[cfg(test)]
pub mod server;

#[cfg(test)]
pub fn set_test_env() {
    use std::env;

    let _ = env::set_var("SFOX_AUTH_TOKEN", "secret");
}
