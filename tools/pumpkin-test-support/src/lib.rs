#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

#[cfg(target_os = "android")]
const TEST_WORK_DIR: &str = "/data/local/tmp/pumpkin_test";

#[cfg(test)]
#[ctor::ctor]
fn init_test_environment() {
    #[cfg(target_os = "android")]
    {
        use std::env;
        let dir = std::env::var("PUMPKIN_TEST_DIR").unwrap_or_else(|_| TEST_WORK_DIR.to_string());
        if let Err(e) = env::set_current_dir(&dir) {
            panic!("Can't switch working directory to {}: {}", dir, e);
        }
        println!(
            "The test working directory has been switched to: {:?}",
            env::current_dir().unwrap()
        );
    }
}
