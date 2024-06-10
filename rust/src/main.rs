use std::io::{stdout, Write};

fn main() {
    let output = stdout();
    write!(&output, "Hello, World!").unwrap();
}
