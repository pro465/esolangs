pub enum I {
    Nop,
    Jmp,
    Dup,
    Inp,
    Out,
    Pass,
    Deq,
    Enq,
    Add,
    Sub,
}

pub fn parse(code: Vec<u8>) -> Vec<I> {
    let mut res = Vec::new();
    for c in code {
        use I::*;

        res.push(match c {
            b'_' => Nop,
            b';' => Jmp,
            b'.' => Out,
            b',' => Inp,
            b':' => Dup,
            b'+' => Add,
            b'-' => Sub,
            b'$' => Deq,
            b'#' => Enq,
            b'/' => Pass,
            _ => continue,
        });
    }
    res
}

fn u<T>(c: Option<T>) -> T {
    c.unwrap_or_else(|| std::process::exit(0))
}

pub fn run(code: Vec<I>) {
    use std::io::{prelude::*, stdin, stdout};
    use I::*;

    let mut stdin = stdin().lock().bytes();
    let mut stdout = stdout();
    let mut ip = 0;
    let mut queue = std::collections::VecDeque::new();

    while ip < code.len() {
        match code[ip] {
            Nop => {}
            Jmp => {
                queue.push_back(ip);
                ip = u(queue.pop_front());
            }
            Pass => {
                let res = u(queue.pop_front());
                queue.push_back(res);
            }
            Enq => queue.push_front(1),
            Deq => {
                u(queue.pop_front());
            }
            Dup => {
                let res = u(queue.pop_front());
                queue.push_front(res);
                queue.push_front(res);
            }
            Inp => {
                let res = stdin.next().unwrap().unwrap() as usize;
                queue.push_front(res);
            }
            Out => {
                let res = u(queue.pop_front());
                stdout.write_all(&[res as u8]).unwrap();
            }
            Add => {
                let a = u(queue.pop_front());
                let b = u(queue.pop_front());
                queue.push_front(a.wrapping_add(b));
            }
            Sub => {
                let a = u(queue.pop_front());
                let b = u(queue.pop_front());
                queue.push_front(a.wrapping_sub(b));
            }
        }
        ip += 1;
    }
}
