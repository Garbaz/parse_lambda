use std::{
    env, fs,
    io::{self, Read},
};

use parse_lambda::lambda2::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = match &args[..] {
        [_, filename] => fs::read_to_string(filename).expect(
            format!("Could not read the file \"{}\"!", filename).as_str(),
        ),
        _ => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .expect("Could not read stdin!");
            buf
        }
    };
    println!(
        "{0}\n--------------------------------\n{0:#?}",
        parse(&input).unwrap()
    );
}
