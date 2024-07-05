/* 
1 
mod struct_with_initial;

fn main() {
    let my_class = struct_with_initial::Class::new(10,5);

    println!("Add: {}", my_class.add());
    println!("Sub: {}", my_class.sub());
    println!("Divide: {}", my_class.divide());
    println!("Multiply: {}", my_class.multiply());
}


*/


/* 

2
mod  struct_without_initial;

fn main() {
    let my_class = struct_without_initial::Class::new();

    let x = 10;
    let y = 5;
    
    println!("Add: {}", my_class.add(x, y));
    println!("Sub: {}", my_class.sub(x, y));
    println!("Divide: {}", my_class.divide(x, y));
    println!("Multiply: {}", my_class.multiply(x, y));
}


*/




/* 
3


mod struct_with_set;

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



*/





/* 
4
mod struct_witout_constructor;


fn main() {
    let x = 10;
    let y = 5;
    
    println!("Add: {}", struct_witout_constructor::Class::add(x, y));
    println!("Sub: {}", struct_witout_constructor::Class::sub(x, y));
    println!("Divide: {}", struct_witout_constructor::Class::divide(x, y));
    println!("Multiply: {}", struct_witout_constructor::Class::multiply(x, y));
}

*/

mod struct_with_set;

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
