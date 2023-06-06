use std::io::{self, prelude::*, Bytes, StdinLock as Inp, StdoutLock as Out};

#[derive(Clone, Copy)]
enum Instr {
    Ref,
    Deref,
    Inp,
    Out,
    Left,
    Right,
    Js(usize),
    Je(usize),
}

pub struct Interpreter {
    prog: Vec<Instr>,
    input: Bytes<Inp<'static>>,
    output: Out<'static>,
    data: Vec<usize>,
    pc: usize,
    pointer: Vec<usize>,
}

impl Interpreter {
    pub fn execute(&mut self) {
        while self.pc < self.prog.len() {
            self.execute_next();
            self.pc += 1;
        }
    }

    fn execute_next(&mut self) {
        use Instr::*;
        let ins = self.prog[self.pc];
        match ins {
            Deref => {
                let c = *self.curr();
                self.pointer.push(c)
            }
            Ref => {
                let idx = self.pointer.pop().unwrap();
                *self.curr() = idx;
            }
            Left => *self.curr_p() -= 1,
            Right => {
                let d = self.curr_p();
                *d += 1;
                if *d >= self.data.len() {
                    self.data.push(0);
                }
            }
            Js(x) => {
                if *self.curr() == 0 {
                    self.pc = x;
                }
            }
            Je(x) => {
                if *self.curr() != 0 {
                    self.pc = x;
                }
            }
            Inp => *self.curr() = usize::from(self.input.next().unwrap().unwrap()),
            Out => {
                let c = (*self.curr()).try_into().unwrap();
                self.output.write_all(&[c]).unwrap()
            }
        }
    }

    fn curr_p(&mut self) -> &mut usize {
        ub(self.pointer.last_mut())
    }

    fn curr(&mut self) -> &mut usize {
        let t = *self.curr_p();
        ub(self.data.get_mut(t))
    }
}

fn ub<T>(x: Option<T>) -> T {
    x.unwrap_or_else(|| panic!("why tf would anyone choose UB?"))
}

pub fn parse(code: Vec<u8>) -> Interpreter {
    Interpreter {
        prog: parse_internal(code),
        input: io::stdin().lock().bytes(),
        output: io::stdout().lock(),
        data: vec![0],
        pc: 0,
        pointer: vec![0],
    }
}

fn parse_internal(code: Vec<u8>) -> Vec<Instr> {
    use Instr::*;
    let mut pos = vec![];
    let mut res = vec![];

    for c in code {
        let r = match c {
            b'[' => {
                pos.push(res.len());
                Js(0)
            }
            b']' => {
                let idx = pos.pop().unwrap();
                res[idx] = Js(res.len());
                Je(idx)
            }
            b'<' => Left,
            b'>' => Right,
            b',' => Inp,
            b'.' => Out,
            b'&' => Ref,
            b'*' => Deref,
            _ => continue,
        };

        res.push(r)
    }

    res
}
