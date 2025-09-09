#[derive(Clone, Debug, PartialEq, Eq)]
struct BinTree {
    left: Option<Box<BinTree>>,
    right: Option<Box<BinTree>>,
}

impl BinTree {
    fn replace(&mut self, pat: &BinTree, rep: &BinTree) {
        if self == pat {
            *self = rep.clone();
        } else {
            self.left.as_mut().map(|x| x.replace(pat, rep));
            self.right.as_mut().map(|x| x.replace(pat, rep));
        }
    }
    fn parse(s: &str) -> Option<BinTree> {
        let s = s.trim();
        if !s.starts_with('(') {
            None
        } else {
            let s = &s[1..];
            let mut level = 1i64;
            let mut end = None;
            for (i, c) in s.char_indices() {
                if c == ')' {
                    level -= 1
                } else if c == '(' {
                    level += 1
                }
                assert!(level >= 0);
                if level == 0 {
                    end = Some(i);
                    break;
                }
            }
            let left_end = end.unwrap_or(s.len());
            let right_start = if left_end + 1 < s.len() {
                Some(left_end + 1)
            } else {
                None
            };
            let left = BinTree::parse(&s[..left_end]);
            let right = right_start.map(|i| BinTree::parse(&s[i..])).flatten();
            let left = left.map(Box::new);
            let right = right.map(Box::new);
            Some(BinTree { left, right })
        }
    }
}

fn run(prog: &str, mut steps: u64) -> bool {
    let mut tree = BinTree::parse(prog).unwrap();
    loop {
        if steps == 0 {
            return true;
        }
        let r = if let Some(r) = &mut tree.right {
            r
        } else {
            break;
        };
        let l = if let Some(l) = &tree.left { l } else { break };
        let pat = &l.left;
        let rep = &l.right;
        if pat.is_none() || rep.is_none() {
            break;
        }
        let (pat, rep) = (pat.as_ref().unwrap(), rep.as_ref().unwrap());
        r.replace(pat, rep);
        tree = *tree.right.unwrap();
        steps -= 1;
    }
    false
}

fn conv(mut num: u128, s: &mut String) -> bool {
    let mut l = 0i64;
    while num > 0 {
        let idx = num as usize & 1;
        s.push([')', '('][idx]);
        l += [-1, 1][idx];
        if l < 0 {
            return false;
        }

        num >>= 1;
    }
    true
}

fn main() {
    let mut c = 0;
    let mut s = String::new();
    for p in 1u128.. {
        s.clear();
        if p.is_power_of_two() {
            println!("{p}: {c}");
            c = 0;
        }
        if !conv(p, &mut s) {
            continue;
        }
        c += 1;
        if run(&s, 100) {
            println!("{s}")
        }
    }
}
