fn main() {

    //Ownership in Variables
    let x = 5;
    let y = x; // Copy is possible for simple data types

    let s1 = String::from("hello");
    let s2 = s1; // This is considered as a "move" operation. Value of s1 is moved to s2 and s1 is no more valid or accessible.

    println!("{}", s1); // Not possible since memory is allocated in heap

    let s3 = s1.clone(); // This is possible.

    // Ownership in Functions
    let s4 = String::from("hello");
    takes_ownership(s4);
    println!("{}", s4); // Not possible since variable s4 is out of scope and ownership is moved to takes_ownership.

    let s5 = gives_ownership();
    let s6 = String::from("hello");
    let s7 = takes_and_gives_back(s6);
    println!("s5 = {}, s7 = {}", s5, s7); // Possible since ownership is taken and given back.

    let s8 = String::from("hello");
    let len = calculate_length(&s8); // References
    println!("The length of '{}' is {}.", s1, len); // Ownership of s8 is not taken, rather than a reference of s8 is used.

    let mut s9 = String::from("hello");

    let s10 = &mut s9; // Possible 
    let s11 = &mut s9; // Not possible because only one mutable reference is allowed.
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}


