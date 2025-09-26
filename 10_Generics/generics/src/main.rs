fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(number_list);

    println!("The largest number is {}", largest);
    
    let number_list = vec![102, 34, 6000, 50, 25, 100, 65];

    let largest = get_largest_generic(number_list);

    println!("The largest number is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest = get_largest_generic(char_list);

    println!("The largest character is {}", largest);

}

fn get_largest(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn get_largest_char(number_list: Vec<char>) -> char {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn get_largest_generic<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &U {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}