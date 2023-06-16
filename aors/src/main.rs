use std::collections::HashMap;
use std::fs;

fn main() {
    let mut args = std::env::args();
    let prog = fs::read_to_string(
        fs::canonicalize(args.nth(1).unwrap_or_else(|| help()))
            .expect("could not canonicalize argument"),
    )
    .expect("could not read file");

    let mut state = State::parse(&prog);

    let lim = args.next().map(|n| n.parse().unwrap_or_else(|_| help()));
    if let Some(lim) = lim {
        for _ in 0..=lim {
            println!("{}", state.step());
        }
    } else {
        loop {
            println!("{}", state.step());
        }
    }
}

fn help() -> ! {
    println!(
        "usage: {} <filename> [max_iter]",
        std::env::current_exe()
            .unwrap_or_else(|_| "aors".into())
            .display()
    );
    std::process::exit(-1);
}

fn is_valid(x: char) -> bool {
    x.is_uppercase() || x.is_lowercase() || x == '$'
}

struct State<'a> {
    inp: String,
    rules: HashMap<char, [&'a str; 2]>,
}

impl<'a> State<'a> {
    fn parse(prog: &'a str) -> Self {
        let mut iter = prog
            .lines()
            .flat_map(|line| {
                line.split_once('#')
                    .map(|i| i.0)
                    .unwrap_or(line)
                    .split_whitespace()
            })
            .filter(|x| !x.is_empty());
        let inp = iter.next().unwrap().to_string();
        let mut rules = HashMap::<_, [_; 2]>::new();
        for rule in iter {
            let parity = rule.as_bytes()[0];
            assert!((b'0'..=b'1').contains(&parity));
            let (sym, repl) = rule[1..].split_once(':').unwrap();
            let sym = sym.chars().next().unwrap();
            assert!(is_valid(sym));
            assert!(repl.chars().all(is_valid));
            rules.entry(sym).or_default()[(parity - b'0') as usize] = repl
        }

        Self { inp, rules }
    }

    fn step(&mut self) -> String {
        let mut parity = 0;
        let mut res = String::new();

        for c in self.inp.chars() {
            if c == '$' {
                res = String::new();
                break;
            }
            res.push_str(self.rules[&c][parity]);
            parity ^= c.is_uppercase() as usize;
        }
        dbg!(parity);

        std::mem::replace(&mut self.inp, res)
    }
}
