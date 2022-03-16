fn main() {
    println!("Hello, world!");
    #[derive(Debug)]
    enum IpAddr{
        V4(String),
        V6(String),
    }
    let home =IpAddr::V4(String::from ("127.0..0.1"));
    let loopback =IpAddr::V6(String::from ("::1"));
    println!("The home addres is {:#?} and the loop back address is {:#?}",home,loopback);

    // ===================================================>>>>>>>>
    // ===================================================>>>>>>>>
    // ===================================================>>>>>>>>
    #[derive(Debug)]
    enum Message{
        Quiet,
        Move{x:i32,y:i32},
        Write(String),
        ChangeColor(i32,i32,i32),

    }
    impl Message{
        fn call(&self){
            println!("{:?}",self);
        }
    }
    let m= Message::Write(String::from("hello"));
    let m2 = Message::Move{x:10,y:20};
    m.call();
    m2.call();
}
