pub struct Class {
    x: i32,
    y: i32,
}

impl Class {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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

