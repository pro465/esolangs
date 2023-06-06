use std::io::{self, Write};

pub fn exec(mut code: Vec<u8>) {
    //println!("{}", String::from_utf8_lossy(&code));

    code.reverse();

    loop {
        match code.pop() {
            Some(b'/') => substitute(&mut code),
            Some(b'\\') => write(code.pop().unwrap()),
            Some(x) => write(x),
            None => break,
        }
    }
}

fn write(c: u8) {
    let mut io = io::stdout();
    io.write_all(&[c]).unwrap();
    io.flush().unwrap();
}

fn substitute(code: &mut Vec<u8>) {
    //println!("{}", String::from_utf8_lossy(&code));
    let pattern = str_before_slash(code);

    if pattern.is_empty() {
        loop {}
    }

    let mut replacement = str_before_slash(code);
    let pattern = into_pattern(pattern);

    replacement.reverse();

    while pattern.replace(code, &replacement) {}
}

fn str_before_slash(code: &mut Vec<u8>) -> Vec<u8> {
    let mut skip = false;
    let mut res = Vec::with_capacity(code.len());

    loop {
        match code.pop() {
            Some(x) => {
                if !skip {
                    if x == b'/' {
                        break;
                    }
                }

                skip = !skip && x == b'\\';

                if !skip {
                    res.push(x);
                }
            }
            None => std::process::exit(0),
        }
    }

    assert!(!skip);

    res
}

fn into_pattern(pattern: Vec<u8>) -> Pattern {
    let mut table = Vec::with_capacity(pattern.len());
    let mut len = 0;

    table.push(0);

    for &x in pattern.iter().skip(1) {
        if x == pattern[len] {
            len += 1;
        } else {
            while len > 0 && x != pattern[len] {
                len = table[len - 1];
            }
        }

        table.push(len);
    }

    Pattern { pattern, table }
}

struct Pattern {
    table: Vec<usize>,
    pattern: Vec<u8>,
}

impl Pattern {
    fn replace(&self, code: &mut Vec<u8>, r: &[u8]) -> bool {
        let mut res = false;
        let mut i = code.len();

        loop {
            let mut j = 0;
            let idx = code[..i].iter().rposition(|&c| {
                if c == self.pattern[j] {
                    j += 1;
                } else {
                    while j > 0 {
                        j = self.table[j - 1];

                        if c == self.pattern[j] {
                            j += 1;
                            break;
                        }
                    }
                }
                j == self.pattern.len()
            });

            if let Some(idx) = idx {
                code.splice(idx..idx + self.pattern.len(), r.iter().copied());
                i = idx;
                res = true;
            } else {
                return res;
            }
        }
    }
}
