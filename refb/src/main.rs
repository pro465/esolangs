use std::fs;

fn main() {
    refb::parse(
        fs::read(
            fs::canonicalize(std::env::args().nth(1).unwrap_or_else(|| help()))
                .expect("could not canonicalize argument"),
        )
        .expect("could not read file"),
    )
    .execute();
}

fn help() -> ! {
    println!(
        "usage: {} <filename>",
        std::env::current_exe()
            .unwrap_or_else(|_| "refb".into())
            .display()
    );
    std::process::exit(-1);
}
