pub struct Class {
    x: i32,
    y: i32,
}

impl Class {
    // Default constructor with default values for x and y
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    // Method to set x
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    // Method to set y
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn add(&self) -> i32 {
        self.x + self.y
    }

    pub fn sub(&self) -> i32 {
        self.x - self.y
    }

    pub fn divide(&self) -> i32 {
        // Handle divide by zero error
        if self.y != 0 {
            self.x / self.y
        } else {
            panic!("Division by zero error!")
        }
    }

    pub fn multiply(&self) -> i32 {
        self.x * self.y
    }
}

