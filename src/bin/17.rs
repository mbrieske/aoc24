advent_of_code::solution!(17);

enum Instruction {
    Adv(u64), //check
    Bxl(u64),
    Bst(u64),
    Jnz(u64), //check
    Bxc,
    Out(u64), //check
    Bdv(u64),
    Cdv(u64),
}

impl Instruction {
    fn from(ins: u8, operant: u64) -> Option<Self> {
        match ins {
            0 => Some(Self::Adv(operant)),
            1 => Some(Self::Bxl(operant)),
            2 => Some(Self::Bst(operant)),
            3 => Some(Self::Jnz(operant)),
            4 => Some(Self::Bxc),
            5 => Some(Self::Out(operant)),
            6 => Some(Self::Bdv(operant)),
            7 => Some(Self::Cdv(operant)),
            _ => panic!("Unknown instruction"),
        }
    }
}

// The value of a combo operand can be found as follows:
// Combo operands 0 through 3 represent literal values 0 through 3.
// Combo operand 4 represents the value of register A.
// Combo operand 5 represents the value of register B.
// Combo operand 6 represents the value of register C.
// Combo operand 7 is reserved and will not appear in valid programs.

fn combo_operant(operant: u64, regs: &[u64]) -> u64 {
    match operant {
        0..=3 => operant,
        4 => regs[0],
        5 => regs[1],
        6 => regs[2],
        _ => panic!("Unknown combo operand"),
    }
}

fn run(instructions: Vec<&Instruction>, reg_a: u64) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::new();
    let mut regs = vec![reg_a, 0, 0];
    let mut ip = 0;

    while let Some(ins) = instructions.get(ip) {
        match ins {
            Instruction::Adv(operant) => {
                //The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
                regs[0] /= 2u64.pow(combo_operant(*operant, &regs) as u32);
                ip += 1;
            }
            Instruction::Bxl(operant) => {
                //The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
                regs[1] ^= *operant;
                ip += 1;
            }
            Instruction::Bst(operant) => {
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
                regs[1] = combo_operant(*operant, &regs) % 8;
                ip += 1;
            }
            Instruction::Jnz(operant) => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
                if regs[0] != 0 {
                    ip = *operant as usize / 2;
                } else {
                    ip += 1;
                }
            }
            Instruction::Bxc => {
                // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
                regs[1] ^= regs[2];
                ip += 1;
            }
            Instruction::Out(operant) => {
                // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
                let o = combo_operant(*operant, &regs) % 8;
                output.push(o);
                ip += 1;
            }
            Instruction::Bdv(operant) => {
                // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
                regs[1] = regs[0] / 2u64.pow(combo_operant(*operant, &regs) as u32);
                ip += 1;
            }
            Instruction::Cdv(operant) => {
                // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
                regs[2] = regs[0] / 2u64.pow(combo_operant(*operant, &regs) as u32);
                ip += 1;
            }
        }
    }
    output
}

pub fn part_one(input: &str) -> Option<String> {
    let (regs, program) = input.split_once("\n\n")?;

    let regs: Vec<_> = regs
        .split('\n')
        .map(|line| {
            let (_, value) = line.split_once(": ").unwrap();
            value.parse::<u64>().unwrap()
        })
        .collect();

    let instructions: Vec<_> = program
        .split_once(": ")?
        .1
        .split(',')
        .collect::<Vec<_>>()
        .chunks(2)
        // .inspect(|chunk| println!("{:?}", chunk))
        .map(|chunk| {
            let (ins, operant) = (chunk[0], chunk[1]);
            let ins = ins.parse::<u8>().unwrap();
            let operant = operant.trim().parse::<u64>().unwrap();
            Instruction::from(ins, operant).unwrap()
        })
        .collect();

    let output = run(instructions.iter().collect(), regs[0]);

    Some(
        output
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(","),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, program) = input.split_once("\n\n")?;

    let program: Vec<u64> = program
        .split_once(": ")?
        .1
        .split(',')
        .map(|v| v.trim().parse::<u64>().unwrap())
        .collect();

    let instructions: Vec<_> = program
        .chunks(2)
        // .inspect(|chunk| println!("{:?}", chunk))
        .map(|chunk| {
            let (ins, operant) = (chunk[0], chunk[1]);
            Instruction::from(ins as u8, operant).unwrap()
        })
        .collect();

    let mut reg_a: u64 = 8_u64.pow((program.len() - 1) as u32);
    let mut output = run(instructions.iter().collect(), reg_a);

    for i in (0..program.len()).rev() {
        while output.get(i) != Some(&program[i]) {
            reg_a += 8_u64.pow(i as u32);
            output = run(instructions.iter().collect(), reg_a);
        }
    }

    while output != program {
        reg_a += 1;
        output = run(instructions.iter().collect(), reg_a);
    }

    Some(reg_a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::*;
    use rstest::rstest;
    use tracing::Level;

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 1), Some(String::from("4,6,3,5,6,3,5,2,1,0")))]
    fn test_part_one(#[case] input: &str, #[case] expected: Option<String>) {
        tracing_init(Level::INFO);
        let result = part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&advent_of_code::template::read_file_part("examples", DAY, 2), Some(117440))]
    fn test_part_two(#[case] input: &str, #[case] expected: Option<u64>) {
        tracing_init(Level::INFO);
        let result = part_two(input);
        assert_eq!(result, expected);
    }
}
