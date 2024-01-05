fn main() {
     let mut string = String::from("Hello from the dynamic string");

     println!("The Dynamic String is {}", string);

     string += " changed beyond this point";

     println!("The Dynamic String is {}", string);

}
