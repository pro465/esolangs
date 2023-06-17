fn main() {
    let mut prog = prod(
        &[[false, false], [false, true]],
        &[true, false, true, false],
    );

    println!("{:?}", &prog);

    let mut ip = 0;
    let n = S * 2;
    let rlen = n + 6;

    let max = prog.iter().map(|i| i.0).max().unwrap_or_default();

    while ip < prog.len().min(max) {
        let (i, j) = prog[ip];
        prog.extend_from_within(ip.saturating_sub(i)..ip);
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
}

const S: usize = 20;
fn prod<const N: usize>(p: &[[bool; N]], inp: &[bool]) -> Vec<(usize, usize)> {
    let n = S * N;
    let rlen = n + 6;
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
    let rlen = n + 6;
    v.extend_from_slice(&[(0, rlen * (p - 1) - 2), (0, n + 2), (S, n)]);
    {
        let prev = v.len();
        for b in r {
            bitconv(v, b, rlen, n, p);
        }
        assert_eq!(v.len() - prev, n)
    }
    v.extend_from_slice(&[(n, 5), (n + 3, 10), (rlen * (p - 1) - 1, rlen + 15)]);
    assert_eq!(v.len() - prev, rlen)
}

fn bitconv(v: &mut Vec<(usize, usize)>, b: bool, rlen: usize, n: usize, p: usize) {
    v.extend_from_slice(&[
        (S, 0),
        (0, S - 2),
        (b as usize * (n + 6), 5),
        (S, 0),
        (0, 2),
        (S, rlen * p + 2),
        (0, rlen * (p - 1) - 2),
        (2, 0),
        (5, 0),
        (n + 14, 4),
        (0, 2),
        (n + 3, 10),
        (rlen * (p - 1) - 1, rlen + 15),
        (2, 0),
        (4, 0),
        (rlen * p + 15, 3),
        (0, 1),
        (rlen * (p - 1) - 1, rlen + 15),
        (1, 1),
        (3, 1),
    ]);
}
