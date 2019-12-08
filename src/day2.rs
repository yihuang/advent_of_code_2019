fn parse_input() -> Vec<u64> {
    let input: &str = include_str!("day2.input");
    input
        .trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn interpreter(prog: &[u64], mem: &mut [u64], mut pc: usize) -> Result<(), &'static str> {
    loop {
        match prog[pc] {
            1 => {
                mem[prog[pc + 3] as usize] =
                    mem[prog[pc + 1] as usize] + mem[prog[pc + 2] as usize];
                pc += 4;
            }
            2 => {
                mem[prog[pc + 3] as usize] =
                    mem[prog[pc + 1] as usize] * mem[prog[pc + 2] as usize];
                pc += 4;
            }
            99 => {
                return Ok(());
            }
            _ => {
                return Err("unknown opcode");
            }
        }
    }
}

fn run(prog: &[u64], noun: u64, verb: u64) -> u64 {
    let mut mem = prog.to_vec();
    mem[1] = noun;
    mem[2] = verb;
    interpreter(prog, &mut mem, 0).unwrap();
    mem[0]
}

pub fn part1() {
    let prog = parse_input();
    println!("{}", run(&prog, 12, 2));
}

pub fn part2() {
    let prog = parse_input();
    for i in 0..100 {
        for j in 0..100 {
            if run(&prog, i, j) == 19_690_720 {
                println!("{}", i * 100 + j);
                break;
            }
        }
    }
}
