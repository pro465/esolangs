use std::fs;

fn main() {
    yolk::exec(
        fs::read_to_string(
            fs::canonicalize(std::env::args().nth(1).unwrap_or_else(|| help()))
                .expect("could not canonicalize argument"),
        )
        .expect("could not read file"),
    )
}

fn help() -> ! {
    println!(
        "usage: {} <filename>",
        std::env::current_exe()
            .unwrap_or_else(|_| "yolk".into())
            .display()
    );
    std::process::exit(-1);
}
