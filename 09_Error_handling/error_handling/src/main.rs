use std::fs::File;

fn main() {
    panic!("crash and burn");

    a();

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let f: Result<File, Error> = File::open("hello.txt");

    let f: File = match f {
        Ok(file: File) => file,
        // Err(error: Error) => panic!("Problem opening the file")
        (error: Error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc: File) => fc,
                Err(e: Error) => panic!("problem creating the file: {:?}", e);
            },
            other_error: ErrorKind => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    }
}

fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't pass in 22!");
    }
}
