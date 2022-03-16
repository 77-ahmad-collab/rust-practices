use std::io;  //Remember that a crate is a collection of Rust source code files.
use std::cmp::Ordering;
use rand::Rng;  // The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
fn main() {
    println!("Hello, world!");

    // <--Guessing Game-->

    // macro to print the string on the screen
   println!("Guess the number!");

    println!("Please input your guess.");
    // <----------------------------------------------------------------------->
   //  create a variable to store the user input, like this:
  //  To make a variable mutable, we add mut before the variable name:
    /* The :: syntax in the ::new line indicates that new is an associated function of the String type. 
    An associated function is a function that’s implemented on a type, in this case String.
     This new function creates a new, empty string. You’ll find a new function on many types,
      because it’s a common name for a function that makes a new value of some kind.*/
    //   <----------------------------------------------------------------------->
       let mut guess = String::new();

//   In full, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new,
//  empty instance of a String. Whew!

// Receive Input -----  io is a module that contains the standard input and output streams.
io::stdin()
        .read_line(&mut guess) // reading user input
         .expect("Failed to read line");
         /*-------------------------------------------- 
        The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data
        without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages
        is how safe and easy it is to use references. You don’t need to know a lot of those details to finish this program. For now, all you
        need to know is that like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to
        make it mutable-----------------------------
        */
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
       println!("You guessed: {}", guess);
    //    storing randomly gnerated number
   let secret_number=rand::thread_rng().gen_range(1..101);
   println!("The secret number is: {}", secret_number);
match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

 let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    //   let mut spaces = "   ";
//     spaces = spaces.len();
// The error says we’re not allowed to mutate a variable’s type:
let guess: u32 = "42".parse().expect("Not a number!");
println!("You guessed: {}", guess);

/////////////////////////////////////////////////////
//SCALER TYPES       /////////////////////////////
//// INTEGER, NUMBER,FLOATING POINT,BOOLEANS ,CHARACTER///////////
////////////////////////////////////////////////////////////

// /To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:

// Wrap in all modes with the wrapping_* methods, such as wrapping_add
// Return the None value if there is overflow with the checked_* methods
// Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
// Saturate at the value’s minimum or maximum values with saturating_* methods


// ////////////////////////////////////////////////////////
// Compound Types
// Compound types can group multiple values into one type. 
// Rust has two primitive compound types: tuples and arrays.


// let tuple =(500,6.4,1);

let a =[1,2,3,4,5];
let mut index =  String::new();
println!("Please enter an index");
io::stdin()
        .read_line(&mut index) // reading user input
         .expect("Failed to read line");
        
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
   let element = a[index];
   println!("The element at index {} is {}",index,element);
   print_name("ahmed".to_owned());
   let number = get_num();
  
   if number!=0{
    println!("The required number is {}",number);
     let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number)
}
//  loop {
//      println!("again");
//  }
 let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
 let mut number = 3;
 while number > 0 {
     println!("{}!", number);
     number -=1;
 }
  let a = [10, 20, 30, 40, 50];

    for element in 1..3 {
        println!("the value is: {}", element);
    }
    let names= ["ahmed","mohamed","ali"];
    for name in names.into_iter(){
        match name{
          "ahmed"=>println!("ahmed"),
            _=>println!("Helloo {}",name),
        }
    }
}
fn print_name(name:String){
    println!("Hello {}",name);
}
fn get_num()->u32{
    60
}
