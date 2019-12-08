fn parse_input() -> Vec<u64> {
    let input: &str = include_str!("day2.input");
    input
        .trim_end()
        .split(',')
        .map(|s| {
            println!("{}", s);
            s.parse().unwrap()
        })
        .collect()
}

fn interpreter(input: &mut [u64], mut pc: usize) -> Result<(), &'static str> {
    loop {
        match input[pc] {
            1 => {
                input[input[pc + 3] as usize] =
                    input[input[pc + 1] as usize] + input[input[pc + 2] as usize];
                pc += 4;
            }
            2 => {
                input[input[pc + 3] as usize] =
                    input[input[pc + 1] as usize] * input[input[pc + 2] as usize];
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

pub fn part1() {
    let mut input = parse_input();
    input[1] = 12;
    input[2] = 2;
    interpreter(&mut input, 0).unwrap();
    println!("{}", input[0]);
}
