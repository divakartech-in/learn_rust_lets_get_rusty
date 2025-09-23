fn main() {
    // If else statements
    
    let number = 5;

    if number < 10 {
        println!("First condition is true");
    } else if number > 10 {
        println!("Second condition is true");
    } else {
        println!("Both conditions are false");
    }

    // Basic loop with break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };

    println!("The result is : {}", result);

    // While loop
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // For loop
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in 1..4 {
        println!("{}!", number);
    }
}
