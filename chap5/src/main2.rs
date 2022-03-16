#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}
impl Rectangle{
    fn area(&self)->u32{
        self.width*self.height
    }
}
fn main(){
    let width1 =30;
    let height1 = 50;
    let rectangle= (30,50);
    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
    println!("the second area is {}", calculate_area(rectangle));
    let rect1 = Rectangle{
        width:30,
        height:50,
    };
    println!("the third area is {}", area_struct(&rect1));
    println!("Our Struct Rectangle is {:#?}===============::",rect1.area());
    // dbg!(&rect1);

}
fn area_struct(rectangle:&Rectangle)->u32{
    rectangle.width * rectangle.height
}


fn area(width:u32,height:u32)->u32{
    width*height
}
fn calculate_area(dimensions:(u32,u32))->u32{
    dimensions.0*dimensions.1
}
// ---------------------------------------------5.1 SECTION-----------------------------------------------------
// =================================================================
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn build_user(email:String,username:String)->User{
//    return User{
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }
// fn main() {
//     println!("Hello, world!");
//       build_user(String::from("ahmad@gmail.com"),String::from("ahmad"));
//     println!("{}",User);
//     // println!("The user details are {}",user1);
// //  let mut user1 = User {
// //         email: String::from("someone@example.com"),
// //         username: String::from("someusername123"),
// //         active: true,
// //         sign_in_count: 1,
// //     };

// //     user1.email = String::from("anotheremail@example.com");
// //     println!("{}", user1.email);
// }
