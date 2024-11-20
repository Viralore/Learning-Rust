#[derive(Debug)] // Automatically generates a Debug trait implementation for easy printing
struct Rectangle {
    height: u32, // Height of the rectangle
    width: u32,  // Width of the rectangle
}

impl Rectangle {
    /// Calculates the area of the rectangle.
    ///
    /// # Returns
    ///
    /// This method returns the area of the rectangle as a `u32` value (height * width).
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Checks if the rectangle has a non-zero width.
    ///
    /// # Returns
    ///
    /// Returns `true` if the width is greater than 0, otherwise `false`.
    fn width(&self) -> bool {
        self.width > 0
    }

    /// Determines if the current rectangle can completely hold another rectangle.
    ///
    /// # Parameters
    ///
    /// - `other`: A reference to another `Rectangle` to compare with.
    ///
    /// # Returns
    ///
    /// Returns `true` if the current rectangle is larger in both width and height than the `other` rectangle.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /// Creates a square with the given size.
    ///
    /// # Parameters
    ///
    /// - `size`: The size of the square’s sides (both width and height).
    ///
    /// # Returns
    ///
    /// Returns a new `Rectangle` where both width and height are equal to `size`.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Create a rectangle with specific dimensions
    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };

    // Print whether the width is greater than 0
    println!("Width > 0 -> {}", rect1.width());
    
    // Print the area of the rectangle
    println!("The area of rectangle is {} square pixels", rect1.area());

    // Print a debug representation of the rectangle
    println!("rect1 is {rect1:#?}");

    // Create another rectangle with smaller dimensions
    let rect2 = Rectangle {
        height: 20,
        width: 25,
    };

    // Check if rect1 can hold rect2
    println!("can rect1 hold rect2? -> {}", rect1.can_hold(&rect2));

    // Create a square with side length of 3
    let sq = Rectangle::square(3);

    // Print the area of the square
    println!("area of square : {}", sq.area());
}
