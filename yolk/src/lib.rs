use core::panic;

#[derive(Clone, PartialEq, Eq)]
enum SExp {
    Ifeq,
    Quote,
    Head,
    Tail,
    Cons,
    This,
    Arg,
    List(Vec<SExp>),
}

impl SExp {
    fn head(self) -> SExp {
        match self {
            SExp::List(mut l) => l.pop().unwrap(),
            _ => panic!("not a list"),
        }
    }
    fn tail(mut self) -> SExp {
        match self {
            SExp::List(ref mut l) => {
                l.pop().unwrap();
                self
            }
            _ => panic!("not a list"),
        }
    }
}
use std::fmt;

impl fmt::Display for SExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ifeq => "ifeq",
                Quote => "quote",
                Head => "head",
                Tail => "tail",
                Cons => "cons",
                This => "this",
                Arg => "arg",
                List(x) => {
                    write!(f, "(")?;
                    let len = x.len();
                    for (i, x) in x.iter().rev().enumerate() {
                        x.fmt(f)?;
                        if i + 1 < len {
                            write!(f, " ")?;
                        }
                    }
                    write!(f, ")")?;
                    return Ok(());
                }
            }
        )
    }
}
use SExp::*;
pub fn exec(p: String) {
    let toks = tokenize(&p);
    let (exp, _) = parse(&toks, false);

    for input in std::io::stdin().lines() {
        let input = input.unwrap();
        let toks = tokenize(&input);
        let (mut inp, _) = parse(&toks, false);

        inp.extend(exp.clone());

        let f = inp.pop().unwrap();

        let res = get(&f.clone(), f, List(inp));

        println!("{}", res);
    }
}

fn tokenize(p: &str) -> Vec<&str> {
    let mut toks = Vec::new();
    for s in p.split_whitespace() {
        let mut start = 0;
        let mut end = 0;

        for c in s.chars() {
            if "()".contains(c) {
                if end > start {
                    toks.push(&s[start..end]);
                }
                toks.push(if c == '(' { "(" } else { ")" });
                start = end + 1;
            } else if !c.is_alphabetic() {
                panic!("unexpected token {c}");
            }
            end += 1;
        }
        if end > start {
            toks.push(&s[start..end]);
        }
    }
    toks
}

fn parse(toks: &[&str], nested: bool) -> (Vec<SExp>, usize) {
    let mut v = Vec::new();
    let mut idx = 0;

    while let Some(&tok) = toks.get(idx) {
        v.push(match tok {
            "ifeq" => Ifeq,
            "quote" => Quote,
            "self" => This,
            "arg" => Arg,
            "head" => Head,
            "tail" => Tail,
            "cons" => Cons,
            "(" => {
                let (list, len) = parse(&toks[idx + 1..], true);
                idx += len;
                List(list)
            }
            ")" if nested => {
                v.reverse();
                return (v, idx + 1);
            }

            s => panic!("unexpected token {s}"),
        });
        idx += 1;
    }
    if nested {
        panic!("expected right parenthesis");
    }
    v.reverse();
    (v, 0)
}

fn eval(this: &SExp, mut e: Vec<SExp>, mut arg: SExp) -> SExp {
    use SExp::*;
    assert!(!e.is_empty());

    match e.pop().unwrap() {
        Quote => e.pop().unwrap(),
        Arg => arg.clone(),
        Head => {
            assert!(!e.is_empty());
            get(this, e.pop().unwrap(), arg).head()
        }

        Tail => {
            assert!(!e.is_empty());
            get(this, e.pop().unwrap(), arg).tail()
        }

        Cons => {
            assert!(e.len() > 1);

            let a = get(this, e.pop().unwrap(), arg.clone());
            let mut b = get(this, e.pop().unwrap(), arg.clone());
            match b {
                List(ref mut l) => l.push(a),
                _ => panic!("cons: not a list"),
            }
            b
        }

        This => {
            assert!(!e.is_empty());
            arg = get(this, e.pop().unwrap(), arg);
            get(this, this.clone(), arg)
        }

        Ifeq => {
            assert!(e.len() > 3);
            let a = get(this, e.pop().unwrap(), arg.clone());
            let b = get(this, e.pop().unwrap(), arg.clone());
            let c = e.pop().unwrap();
            let d = e.pop().unwrap();
            if a == b {
                get(this, c, arg.clone())
            } else {
                get(this, d, arg.clone())
            }
        }

        l => {
            e.push(l);
            panic!("cannot evaluate expression {}", List(e));
            //let l = eval(&List(l.clone()), l, List(e));
            //get(this, l, arg)
        }
    }
}

fn get(this: &SExp, e: SExp, arg: SExp) -> SExp {
    match e {
        SExp::List(l) => eval(this, l, arg),
        SExp::Arg => arg.clone(),
        x => x,
    }
}
