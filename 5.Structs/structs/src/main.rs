struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool, 
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("user1@gmail.com"),
        username: String::from("username1"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("user1");

    let user2 = build_user(String::from("user2@gmail.com"), String::from("user2"));

    let user3 = User {
        email: String::from("user3@gmail.com"),
        username: String::from("user3"),
        ..user2
    };

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!("Rect: {:#?}", rect);

    // println!("The area of the rectangle is {} square pixels.", area(&rect));
    println!("The area of the rectangle is {} square pixels.", rect.area());
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }