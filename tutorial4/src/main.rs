//  Scaler and Compound variable types
fn main () {

    // Scaler Types

    let x: u32 = 4294967295; // integer unsigned

    let f: f32 = 13.4; //Floating point variable f32 f64

    let b: bool = true; // Boolean variable

    let c: char =  'c'; // characters variable
    println!("interger: {}\nfloat: {}\nboolean: {}\nchar: {}\n", x, f, b, c);

    // Scaler Types

    // Compound Types
    let mut tup: (i32, bool, char) = (32, false, 's'); // Mutable tuple
    println!("Tup1 {}", tup.1);
    tup.0 = 10;

    let mut arr: [i32; 5] = [0,1,2,3,4];
    arr[3] = 10; 
    println!("Array value at index 3 {}", arr[3]);
    
    let mut arr1: [i32; 5] = [0,1,2,3,4];
    arr1[3] += 10; 
    println!("Array1 value at index 3 {}", arr1[3]);
    
    // Compound Types



}
