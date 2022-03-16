

fn main() {
      // clone function copies all the heap data
    let s1 = String::from("hello world");

    // let (s2,length) = calculate_length(s1);

    // println!("The length of '{}' is. {}", s2,length);
    // Mutable References
    let mut name = String::from("hello");
    change(&mut name);
    println!("{}",name);
    println!("==============================================");
    println!("==============================================");
    let wordof =  firstword(&s1);
    println!("{}",wordof);
}
fn firstword(s: &String)->&str {
    let bytes = s.as_bytes();
    println!("{}",bytes[0]);
    for (i, &item) in bytes.iter().enumerate() {
        println!("item {} and its location is {} and the index is {}",item,&item,i);
         if item == b' ' {
            return &s[0..i];
        }   
    }
   &s[..]
}
fn change(some_string: &mut String)  {
    some_string.push_str(", world 89");
}
// fn calculate_length(s: String) ->  (String,usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }
// The scope in which the variable s is valid is the same as any function parameter’s scope
// , but the value pointed to by the reference is not dropped when s stops being used because s doesn’t
//  have ownership. When functions have references as parameters instead of the actual values, we won’t
//  need to return the values in order to give back ownership, because we never had ownership.

// here is the concept down below
// =======================================================
//     let s1 = String::from("hello");======================
//                                    ======================
//     let len = calculate_length(&s1);=====================
//     fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } 

// here exactly what we are doing is to passing areference of s1 content to function parametere
// function is borrowing the value then returning it


// Mutable references have one big restriction: you can have only one mutable reference 
// to a particular piece of data at a time. This code that attempts to create two mutable
// / references to s will fail:





// // =Two or more pointers access the same data at the same time.
// At least one of the pointers is being used to write to the data.
// There’s no mechanism being used to synchronize access to the data.
// Data races cause undefined behavior and can be difficult to diagnose and fix when you’re 
// trying to track them
//  down at runtime; Rust prevents this problem by refusing to compile code with data races!