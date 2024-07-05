#  class mit constructor der zwei parameter erwartet bei der initialisierung


```rust

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





fn main() {
    let my_class = Class::new(10,5);

    println!("Add: {}", my_class.add());
    println!("Sub: {}", my_class.sub());
    println!("Divide: {}", my_class.divide());
    println!("Multiply: {}", my_class.multiply());
}

```




# class mit constructor der keine parameter erwartet bei der initialisierung



```rust
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

fn main() {
    let my_class = struct_without_initial::Class::new();

    let x = 10;
    let y = 5;
    
    println!("Add: {}", my_class.add(x, y));
    println!("Sub: {}", my_class.sub(x, y));
    println!("Divide: {}", my_class.divide(x, y));
    println!("Multiply: {}", my_class.multiply(x, y));
}
```



# class mit constructor der keine parameter erwartet bei der initialisierung aber set method hat

```rust


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






fn main() {
    let mut my_class = struct_with_set::Class::new();
    
    // Set values for x and y
    my_class.set_x(10);
    my_class.set_y(5);
    
    println!("Add: {}", my_class.add());
    println!("Sub: {}", my_class.sub());
    println!("Divide: {}", my_class.divide());
    println!("Multiply: {}", my_class.multiply());
}

```
