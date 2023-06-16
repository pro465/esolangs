fn main() {
    let mut prog = prod(&[[false, false], [false, true]], &[true, false]);
    let mut ip = 0;
    let max = prog.iter().map(|i| i.0).max().unwrap_or_default();
    while ip < prog.len() {
        let (i, j) = prog[ip];
        prog.extend_from_within(ip - i..ip);
        ip += j + 1;
    }

    if max >= prog.len() {
        return;
    }

    prog.splice(0..ip - max, []);
    while let Some(&(i, j)) = prog.get(max) {
        prog.extend_from_within(max - i..i);
        prog.splice(0..prog.len().max(j + 1), []);
    }

    println!("{:?}", prog);
}

const S: usize = 22;
fn prod<const N: usize>(p: &[[bool; N]], inp: &[bool]) -> Vec<(usize, usize)> {
    let n = S * N;
    let rlen = n + 6;
    let mut res = vec![(0, rlen * p.len() + 2)];
    for r in p.iter().copied().rev() {
        rule(&mut res, r, p.len());
    }
    assert_eq!(res.len() - 1, rlen * p.len());
    res.push((S, 0));
    for c in inp {
        bitconv(&mut res, *c, rlen, n, p.len());
    }
    res
}

fn rule<const N: usize>(v: &mut Vec<(usize, usize)>, r: [bool; N], p: usize) {
    let prev = v.len();
    let n = S * N;
    let rlen = n + 6;
    v.extend_from_slice(&[(0, rlen * (p - 1) - 2), (0, n + 2), (0, n)]);
    {
        let prev = v.len();
        for b in r {
            bitconv(v, b, rlen, n, p);
        }
        assert_eq!(v.len() - prev, n)
    }
    v.extend_from_slice(&[(n, 4), (n + 3, 11), (rlen * (p - 1) - 2, rlen + 17)]);
    assert_eq!(v.len() - prev, rlen)
}

fn bitconv(v: &mut Vec<(usize, usize)>, b: bool, rlen: usize, n: usize, p: usize) {
    v.extend_from_slice(&[
        (0, S - 2),
        (b as usize * (n + 6), 6),
        (S, 0),
        (0, 3),
        (S, 0),
        (0, rlen * p + 2),
        (0, rlen * (p - 1) - 2),
        (3, 0),
        (6, 0),
        (n + 15, 5),
        (0, 3),
        (n + 3, 11),
        (rlen * (p - 1) - 2, rlen + 17),
        (0, rlen * (p - 1) - 2),
        (3, 0),
        (5, 0),
        (rlen * p + 17, 3),
        (0, 1),
        (rlen * (p - 1) - 2, rlen + 17),
        (1, 1),
        (3, 1),
        (S, 0),
    ]);
}
