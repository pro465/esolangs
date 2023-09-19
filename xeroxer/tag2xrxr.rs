fn main() {
    let prod: &[&[usize]] = &[&[1, 2], &[0], &[0, 0, 0]];
    let input = &[0; 9][..];
    let m = 2;
    let prog = trans(prod, input, m);
    println!("{:?}", prog);
}

/*
 * overview: each symbol copies itself until it is "activated" or destroyed by the production table being near it.
 * an activated symbol first copies its corresponding production, and arranges for the production
 * table to be recreated, with it being set such that it destroys the next m - 1 symbols.  each
 * act (destroying each symbol, and recreating the table) takes 1 full cycle of the whole "queue"
 * rotating. so, an m-tag system taking O(t) time would be emulated in O(t^2) time.
 *
 * |x| = length of x
 * (x) = quote x = (_, |x|) x (|x|, _)
 * prod = (0, 0) * (|sym| + 1) <jump to @y> (0, |rules| + |sym| + 2) rules (0, 0) * (|sym| + 1) <jump to @y> (|sym| + 2, 0) <copy till (0, |rules| + |sym| + 2)> @y: (0, 2)
 * rules = [(rule)]
 * rule = [sym]
 * sym = <copy symbol> <jump to next copier> <copy rule> <copy prod> (( <copy till (0, |rules| + |sym| + 2)> <set m> )) (0, 1)
 * set 1 = (0, 2)
 * set m = (|prod| - 1, |set m - 1|) set m - 1 (|set m - 1|, |sym| + 1)
*/
fn trans(prod: &[&[usize]], input: &[usize], m: usize) -> Vec<(usize, usize)> {
    let map = construct(m, prod);
    let sym_len = get_sym_len(m);
    let footer_len = get_footer_len(sym_len);
    let mut res = vec![(0, 0); sym_len + 1];
    let rules_len = get_rules_len(&map, m);
    let y = rules_len + footer_len + 1 - 1;
    res.extend_from_slice(&[(0, y), (0, rules_len + sym_len + 2)]);

    for (i, &rule) in prod.iter().enumerate() {
        let rule_len = sym_len * rule.len();
        let to_end = map.get(i + 1).map(|&i| i).unwrap_or(0) + footer_len + 2;
        res.push((0, rule_len));
        for &sym in rule {
            sym_convert(&map, &mut res, m, sym);
        }
        res.push((rule_len, to_end));
    }

    for _ in 0..sym_len + 1 {
        res.push((0, 0));
    }
    res.extend_from_slice(&[
        (0, y),
        (sym_len + 2, 0),
        (rules_len + sym_len + 4, 4),
        (0, 2),
    ]);

    for &sym in input {
        sym_convert(&map, &mut res, m, sym);
    }
    res
}
fn sym_convert(map: &[usize], v: &mut Vec<(usize, usize)>, m: usize, sym: usize) {
    let sym_len = get_sym_len(m);
    let rules_len = get_rules_len(map, m);
    let prod_len = get_prod_len(rules_len, sym_len, m);

    v.extend_from_slice(&[
        (sym_len, 0),
        (0, sym_len - 2),
        (map[sym] + get_footer_len(sym_len) + 2, 0),
        (get_footer_len(sym_len) + rules_len + 4, 0),
    ]);
    convert_m(v, m, sym_len, rules_len, prod_len);
    v.push((0, 1))
}

fn convert_m(
    v: &mut Vec<(usize, usize)>,
    m: usize,
    sym_len: usize,
    rules_len: usize,
    prod_len: usize,
) {
    let m_len = get_m_len(m);
    v.extend_from_slice(&[(0, m_len + 3), (0, m_len + 1), (rules_len + sym_len + 4, 4)]);
    for m in (2..=m).rev() {
        let m_len = get_m_len(m - 1);
        v.push((prod_len - 1, m_len));
    }
    v.push((0, 2));
    for m in 2..=m {
        let m_len = get_m_len(m - 1);
        v.push((m_len, sym_len + 1));
    }
    v.extend_from_slice(&[(m_len + 1, 1), (m_len + 3, 0)]);
}

fn construct(m: usize, prod: &[&[usize]]) -> Vec<usize> {
    let mut res = Vec::new();
    let sym_len = get_sym_len(m);
    for i in prod {
        res.push(i.len() * sym_len + 2);
    }
    let mut acc = 0;
    for i in res.iter_mut().rev() {
        acc += *i;
        *i = acc;
    }
    res
}

fn get_m_len(m: usize) -> usize {
    m * 2 - 1
}

fn get_sym_len(m: usize) -> usize {
    get_m_len(m) + 10
}

fn get_rules_len(map: &[usize], m: usize) -> usize {
    map[0]
}

fn get_footer_len(sym_len: usize) -> usize {
    sym_len + 5
}

fn get_prod_len(rules_len: usize, sym_len: usize, m: usize) -> usize {
    2 * (sym_len + 2) + rules_len + 4
}
