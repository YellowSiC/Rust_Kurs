pub struct Class;

impl Class {
    // Default constructor with default values for x and y
    pub fn new() -> Self {
        Self
    }

    pub fn add(&self, x: i32, y: i32) -> i32 {
        x + y
    }

    pub fn sub(&self, x: i32, y: i32) -> i32 {
        x - y
    }

    pub fn divide(&self, x: i32, y: i32) -> i32 {
        // Handle divide by zero error
        if y != 0 {
            x / y
        } else {
            panic!("Division by zero error!")
        }
    }

    pub fn multiply(&self, x: i32, y: i32) -> i32 {
        x * y
    }
}

