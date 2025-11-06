use std::io;
fn main() {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let path = input.trim();
            match std::fs::read(path) {
                Ok(_) => println!("success"),
                Err(_) => println!("failure"),
            }
        }
        Err(_) => println!("failure"),
    }
}
