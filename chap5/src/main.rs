struct Rectangle{
    width:u32,
    height:u32, 
}
impl Rectangle{
  fn can_hold(&self,child:&Rectangle)->bool{
    self.width > child.width && self.height > child.height
  }
}
fn main(){
  let rect1 = Rectangle{
    width:30,
    height:50,
  };
  let rect2 = Rectangle{
    width:10,
    height:40,
  };
  let rect3 = Rectangle{
    width:60,
    height:45,
  };
  println!("can rect1 hold rect2? {}",rect1.can_hold(&rect2));
  println!("can rect1 hold rect3? {}",rect1.can_hold(&rect3));
}
// All functions defined within an impl block are called associated functions because they’re associated