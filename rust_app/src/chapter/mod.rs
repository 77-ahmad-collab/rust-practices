
   pub mod chap9;
  use std::collections::HashMap;
pub fn vec(){
  // let v: Vec<i32> = Vec::new();
  let mut v = vec![9, 2, 3];
   v.push(5);
  //   v.push(6);
  //   v.push(7);
  //   v.push(8);
    let third = &v[2];
    println!("The third element is {}", third);
    match v.get(100){
      Some(element)=>println!("The third element is {}", element),
      None=>println!("There is no third element at that index."),
    };
   for i in &mut v{
     *i +=50;
     if *i %2==0{
       println!("Condition satsisfied where{}", i);
     }
    // if let Some(i) >= Some(55){
    //    println!(" the value that is{}", i);
    //  }
     println!("The value is: {}", *i);
   }
      // let hello = String::from("السلام عليكم");
      let mut str1 = String::from("Dobrý den");
      str1.push_str("added");
      println!("The updated String is  {}", str1);
       let s1 = String::from("Hello, ");
      let s2 = String::from("world!");
      let s3 = s1 + &s2;
      println!("The concatenated string is {}", s3);
      println!("The ownership of s2 is {}", s2);
       let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    // let answer = &s[0];
    println!("The formated string is {}",&s[0..3]);
    // ------->>>>>>Creating a Hash Map
        let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("red")).or_insert(230);
    println!("{:?}", scores);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
 
    // use chap9::hello;
}