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
    let my_class = Class::new();

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
    let mut my_class = Class::new();
    
    // Set values for x and y
    my_class.set_x(10);
    my_class.set_y(5);
    
    println!("Add: {}", my_class.add());
    println!("Sub: {}", my_class.sub());
    println!("Divide: {}", my_class.divide());
    println!("Multiply: {}", my_class.multiply());
}

```

#  class ohne constructor


```rust
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


fn main() {
    let x = 10;
    let y = 5;
    
    println!("Add: {}", Class::add(x, y));
    println!("Sub: {}", Class::sub(x, y));
    println!("Divide: {}", Class::divide(x, y));
    println!("Multiply: {}", Class::multiply(x, y));
}



```


# abstract class



```rust


// Trait für grundlegende mathematische Operationen
pub trait AbstractClass<T> {
    fn add(&self, other: T) -> T;
    fn subtract(&self, other: T) -> T;
    fn multiply(&self, other: T) -> T;
    fn divide(&self, other: T) -> T;
}

// Implementierung des Traits für den Typ f64
impl AbstractClass<f64> for f64 {
    fn add(&self, other: f64) -> f64 {
        self + other
    }

    fn subtract(&self, other: f64) -> f64 {
        self - other
    }

    fn multiply(&self, other: f64) -> f64 {
        self * other
    }

    fn divide(&self, other: f64) -> f64 {
        self / other
    }
}

// Beispielnutzung
fn main() {
    let a: f64 = 10.0;
    let b: f64 = 2.0;

    println!("Addition: {}", a.add(b));        // 12.0
    println!("Subtraktion: {}", a.subtract(b)); // 8.0
    println!("Multiplikation: {}", a.multiply(b)); // 20.0
    println!("Division: {}", a.divide(b));     // 5.0
}




```

# list test



```rust

use std::any::Any; // Importieren der Any Trait aus dem Standardbibliothek

fn main() {
    let variables = create_variables(); // Variablen erzeugen
    for var in variables { // Schleife über jede Variable
        print_variable(&*var); // Variable ausgeben
    }
}

fn create_variables() -> Vec<Box<dyn Any>> {
    let mut variables: Vec<Box<dyn Any>> = Vec::new(); // Leere Vektor für verschiedene Datentypen

    variables.push(Box::new(42i32));          // Integer hinzufügen
    variables.push(Box::new(3.14f64));        // Float hinzufügen
    variables.push(Box::new(true));           // Boolean hinzufügen
    variables.push(Box::new('c'));            // Char hinzufügen
    variables.push(Box::new("Hello, Rust!")); // String hinzufügen

    variables // Vektor zurückgeben
}

fn print_variable(var: &dyn Any) {
    if let Some(value) = var.downcast_ref::<i32>() { // Überprüfen und Ausgeben des Integer-Typs
        println!("Integer: {}", value);
    } else if let Some(value) = var.downcast_ref::<f64>() { // Überprüfen und Ausgeben des Float-Typs
        println!("Float: {}", value);
    } else if let Some(value) = var.downcast_ref::<bool>() { // Überprüfen und Ausgeben des Boolean-Typs
        println!("Boolean: {}", value);
    } else if let Some(value) = var.downcast_ref::<char>() { // Überprüfen und Ausgeben des Char-Typs
        println!("Char: {}", value);
    } else if let Some(value) = var.downcast_ref::<&str>() { // Überprüfen und Ausgeben des String-Typs
        println!("String: {}", value);
    } else {
        println!("Unbekannter Typ"); // Wenn der Typ unbekannt ist
    }
}



```

