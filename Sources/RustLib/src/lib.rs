mod sidekick;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn print_hello_rust();
        fn get_a_value_from_rust() -> i32;
        fn get_a_string_from_rust() -> String;
        fn print_string_from_swift(s: String);
    }

    #[swift_bridge::bridge(swift_repr = "struct")]
    pub struct User {
        nickname: String,
    }

    extern "Rust" {
        fn get_user() -> User;
    }

    extern "Swift" {
        fn print_from_swift();
        fn print_number(x: i32);
    }
}

fn print_hello_rust() {
    println!("Hello from Rust");
    ffi::print_from_swift();
    ffi::print_number(11);
}

fn get_a_value_from_rust() -> i32 {
    42
}

fn get_a_string_from_rust() -> String {
    String::from("String from Rust")
}

fn print_string_from_swift(s: String) {
    println!("{}", s);
}

fn get_user() -> ffi::User {
    ffi::User {
        nickname: String::from("V"),
    }
}
