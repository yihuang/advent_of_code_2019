use std::collections::HashMap;
use string_interner::{StringInterner, Sym};

fn parse_input() -> (Vec<(Sym, Sym)>, StringInterner<Sym>) {
    let mut interner = StringInterner::default();
    let input: &str = include_str!("day6.input");
    (
        input
            .lines()
            .map(|line| {
                let mut parts = line.split(')');
                (
                    interner.get_or_intern(parts.next().unwrap()),
                    interner.get_or_intern(parts.next().unwrap()),
                )
            })
            .collect::<Vec<_>>(),
        interner,
    )
}

pub fn part1() {
    let mut orbits: HashMap<Sym, (Sym, usize)> = HashMap::new();
    let (pairs, _interner) = parse_input();
    for (a, b) in pairs {
        // b orbits a

        // first, longgest orbit path of b = a + 1
        // FIXME better complexity
        let path_b = if let Some((v, l)) = orbits.get(&a) {
            (*v, l + 1)
        } else {
            (a, 1)
        };

        // second, extend the orbit paths connected to b
        for (_, (y, l)) in orbits.iter_mut() {
            if y == &b {
                // extend by one
                *l += path_b.1;
                *y = path_b.0;
            }
        }
        orbits.insert(b, path_b);
    }

    println!("{}", orbits.iter().map(|(_, (_, l))| l).sum::<usize>());
}

fn common_ansesters(map: &HashMap<Sym, Sym>, a: Sym, b: Sym) -> Option<(usize, usize)> {
    let mut path: HashMap<Sym, usize> = HashMap::new();
    {
        let mut cursor = a;
        let mut l = 0;
        while let Some(x) = map.get(&cursor) {
            path.insert(*x, l);
            l += 1;
            cursor = *x;
        }
    }
    {
        let mut cursor = b;
        let mut l: usize = 0;
        while let Some(x) = map.get(&cursor) {
            if let Some(l2) = path.get(x) {
                // crossed
                return Some((l, *l2));
            }
            l += 1;
            cursor = *x;
        }
    }
    None
}

pub fn part2() {
    let (pairs, mut interner) = parse_input();
    let (len1, len2) = common_ansesters(
        &pairs
            .into_iter()
            .map(|(a, b)| (b, a))
            .collect::<HashMap<_, _>>(),
        interner.get_or_intern("YOU"),
        interner.get_or_intern("SAN"),
    )
    .unwrap();
    println!("{}", len1 + len2);
}
