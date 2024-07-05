pub struct Class;

impl Class {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    pub fn sub(x: i32, y: i32) -> i32 {
        x - y
    }

    pub fn divide(x: i32, y: i32) -> i32 {
        if y != 0 {
            x / y
        } else {
            panic!("Division by zero error!")
        }
    }

    pub fn multiply(x: i32, y: i32) -> i32 {
        x * y
    }
}


