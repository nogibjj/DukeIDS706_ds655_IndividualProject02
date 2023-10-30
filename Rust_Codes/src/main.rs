fn main() {
    // Declare a variable with type inference
    let x = 5;

    // Declare a mutable variable with type annotation
    let mut y: i32 = 10;

    // Print the values of x and y
    println!("x = {}, y = {}", x, y);

    // Shadow the value of y with a new value
    let y = "hello";

    // Print the new value of y
    println!("y = {}", y);

    // Declare a constant
    const PI: f32 = 3.14159;

    // Declare an array
    let a = [1, 2, 3, 4, 5];

    // Declare a vector
    let mut v = vec![1, 2, 3];

    // Add an element to the vector
    v.push(4);

    // Iterate over the elements of the vector
    for i in &v {
        println!("{}", i);
    }

    // Declare a struct
    struct Point {
        x: f32,
        y: f32,
    }

    // Declare an instance of the struct
    let p = Point { x: 1.0, y: 2.0 };

    // Print the values of the struct fields
    println!("p.x = {}, p.y = {}", p.x, p.y);

}