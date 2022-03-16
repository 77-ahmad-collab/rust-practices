fn main(){
  let config_max = Some(2);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
  // ======section 6.2 ENDED
  // fn plus_one(x: Option<i32>) -> Option<i32> {
  //       match x {
  //           None => {
  //             println!("None");
  //             None}
  //           Some(i) =>{
  //             println!("{:?}",Some(i+1));
  //              Some(i + 1)},
  //       }
  //   }
  //      let five = Some(5);
  //       let six = plus_one(five);
  //       let none = plus_one(None);
  //      println!("the expected value is {:?}",five);
}