use bevy::prelude::*;

const GREETING: &str = "hello world!";

fn hello_world() {
    println!("{}", GREETING);
}

fn main() {
    App::new().add_systems(Startup, hello_world).run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting() {
        assert_eq!(GREETING, "hello world!");
    }
}
