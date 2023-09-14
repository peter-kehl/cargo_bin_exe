#[test]
fn show_test_crate_name() {
    //panic!("CARGO_CRATE_NAME: {}", std::env!("CARGO_CRATE_NAME")); // -> simple_integration_test - OK
}

#[test]
fn show_test_crate_binary_path() {
    panic!(
        "test crate name: {}",
        std::env!("CARGO_BIN_EXE_simple_integration_test")
    );
}
