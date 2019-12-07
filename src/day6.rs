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

pub fn main() {
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

    println!(
        "count: {}",
        orbits.iter().map(|(_, (_, l))| l).sum::<usize>()
    );
}
