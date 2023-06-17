// reference interpreter
use std::fs;

fn main() {
    exec(parse(
        fs::read_to_string(
            fs::canonicalize(std::env::args().nth(1).unwrap_or_else(|| help()))
                .expect("could not canonicalize argument"),
        )
        .expect("could not read file"),
    ))
}

fn help() -> ! {
    println!(
        "usage: {} <filename>",
        std::env::current_exe()
            .unwrap_or_else(|_| "xeroxer".into())
            .display()
    );
    std::process::exit(-1);
}

fn exec(mut prog: Vec<(usize, usize)>) {
    //println!("{:?}", &prog);
    //let n = S * 2;
    //let rlen = n + 7;

    let mut ip = 0;

    let max = prog.iter().map(|i| i.0).max().unwrap_or_default();

    while ip < prog.len().min(max) {
        let (i, j) = prog[ip];
        prog.extend_from_within(ip.saturating_sub(i)..ip);
        /*if i == S {
            println!("copy {:?}", &prog[ip - i..ip]);
            std::thread::sleep_ms(1000);
        }
        if j == rlen * 2 + 2 {
            println!("after copy")
        }
        if (i, j) == (1, 1) {
            println!("ready")
        }*/
        ip += j + 1;
    }

    if max > ip {
        return;
    }

    prog.splice(0..ip - max, []);

    while let Some(&(i, j)) = prog.get(max) {
        prog.extend_from_within(max - i..max);
        prog.splice(0..prog.len().min(j + 1), []);
        // dbg!(prog.len());
    }

    println!("half-baked, (or rather, cut) result: {:?}", prog);
}

fn parse(p: String) -> Vec<(usize, usize)> {
    let mut t = 0;
    let mut v = Vec::with_capacity(p.split_whitespace().count());

    for (i, x) in p.split_whitespace().enumerate() {
        let x = x.parse().expect("could not parse file");
        if i & 1 == 1 {
            if i / 2 < t {
                panic!("undefined behaviour detected @ {}", i - 1);
            }
            v.push((t, x));
        }
        t = x;
    }
    v
}
