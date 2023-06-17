fn main() {
    let mut prog = prod(&[[false, false], [false, true]], &[true, false]);

    println!("{:?}", &prog);
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
    }

    // println!("{:?}", prog);
}

const S: usize = 21;
fn prod<const N: usize>(p: &[[bool; N]], inp: &[bool]) -> Vec<(usize, usize)> {
    let n = S * N;
    let rlen = n + 7;
    let mut res = vec![(0, rlen * p.len() + 2)];
    for r in p.iter().copied().rev() {
        rule(&mut res, r, p.len());
    }
    assert_eq!(res.len() - 1, rlen * p.len());
    for c in inp {
        bitconv(&mut res, *c, rlen, n, p.len());
    }
    res
}

fn rule<const N: usize>(v: &mut Vec<(usize, usize)>, r: [bool; N], p: usize) {
    let prev = v.len();
    let n = S * N;
    let rlen = n + 7;
    v.extend_from_slice(&[(0, rlen * (p - 1) - 2), (0, n + 3), (S, 0), (0, n)]);
    {
        let prev = v.len();
        for b in r {
            bitconv(v, b, rlen, n, p);
        }
        assert_eq!(v.len() - prev, n)
    }
    v.extend_from_slice(&[(n, 5), (n + 4, 11), (rlen * (p - 1) - 1, rlen + 16)]);
    assert_eq!(v.len() - prev, rlen)
}

fn bitconv(v: &mut Vec<(usize, usize)>, b: bool, rlen: usize, n: usize, p: usize) {
    v.extend_from_slice(&[
        (S, 0),
        (0, S - 2),
        (b as usize * (n + 7), 6),
        (S, 0),
        (0, 3),
        (S, 0),
        (0, rlen * p + 2),
        (0, rlen * (p - 1) - 2),
        (3, 0),
        (6, 0),
        (n + 16, 4),
        (0, 2),
        (n + 4, 11),
        (rlen * (p - 1) - 1, rlen + 16),
        (2, 0),
        (4, 0),
        (rlen * p + 16, 3),
        (0, 1),
        (rlen * (p - 1) - 1, rlen + 16),
        (1, 1),
        (3, 1),
    ]);
}
