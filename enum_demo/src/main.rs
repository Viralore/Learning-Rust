// Define an enum `Message` with different variants, each representing a unique type of message.
enum Message {
    Quit, // Variant with no associated data.
    Move { x: i32, y: i32 }, // Variant with named fields to represent movement in 2D space.
    Write(String), // Variant containing a String as associated data.
    ChangeColour(i32, i32, i32), // Variant with a tuple of integers to represent RGB values.
}

// Implement methods for the `Message` enum.
impl Message {
    // Define a method `call` for the `Message` enum.
    // This currently just prints a test message. In real-world usage, it could perform actions based on the variant.
    fn call(&self) {
        println!("Enum test");
    }
}

fn main() {
    // Create an instance of the `Write` variant, containing a String message.
    let m = Message::Write(String::from("Hello"));

    // Call the `call` method on the instance. This demonstrates how methods can be invoked on enums.
    m.call();
}
