// Character 	Meaning
// > 	Increment the data pointer (to point to the next cell to the right).
// < 	Decrement the data pointer (to point to the next cell to the left).
// + 	Increment (increase by one) the byte at the data pointer.
// - 	Decrement (decrease by one) the byte at the data pointer.
// . 	Output the byte at the data pointer.
// , 	Accept one byte of input, storing its value in the byte at the data pointer.
// [ 	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
// ] 	If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.

use std::{
    collections::HashMap,
    io::{self, Write, stdin, Read},
};

fn main() {
    let src = "++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++.>+.+++++++..+++.<<++.>+++++++++++++++.>.+++.------.--------.<<+.<.";
    let bf_instructions: Vec<char> = src.chars().collect();
    let mut bf_instruction_ptr: usize = 0;
    let bf_brackets: HashMap<usize, usize> = find_brackets(&bf_instructions);

    let mut bf_memory: Vec<i64> = vec![0; 32];
    let mut bf_memory_ptr: usize = 0;

    while bf_instruction_ptr < bf_instructions.len() {
        match bf_instructions[bf_instruction_ptr] {
            '>' => {
                bf_memory_ptr += 1;
                if bf_memory_ptr >= bf_memory.len() {
                    bf_memory.push(0);
                }
            }
            '<' => bf_memory_ptr -= 1,
            '+' => bf_memory[bf_memory_ptr] = bf_memory[bf_memory_ptr] + 1,
            '-' => bf_memory[bf_memory_ptr] = bf_memory[bf_memory_ptr] - 1,
            '.' => {
                let printed_char =
                    std::char::from_u32(bf_memory[bf_memory_ptr] as u32).expect(&format!(
                        "Panic: Unknow Unicode {} at ptr {}.",
                        bf_memory[bf_memory_ptr] as u32, bf_instruction_ptr
                    ));
                print!("{}", printed_char);
                io::stdout().flush().unwrap();
            }
            ',' => {
                io::stdout().flush().unwrap();
                let mut ib = [0u8];
                stdin().read_exact(&mut ib).unwrap();
                bf_memory[bf_memory_ptr] = ib[0] as i64;
            }
            '[' => {
                if bf_memory[bf_memory_ptr] == 0 {
                    bf_instruction_ptr = bf_brackets[&bf_instruction_ptr]
                }
            }
            ']' => {
                if bf_memory[bf_memory_ptr] != 0 {
                    bf_instruction_ptr = bf_brackets[&bf_instruction_ptr]
                }
            }
            '\u{26}' => {
                print!("\n<{}> |", bf_memory_ptr);
                for x in &bf_memory {
                    print!(" {} |", x)
                }
                println!();
            }
            _ => (),
        }
        bf_instruction_ptr += 1;
    }
}

fn find_brackets(src: &Vec<char>) -> HashMap<usize, usize> {
    let mut bf_brackets: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;
    while i < src.len() {
        match src[i] {
            '[' => {
                let mut p = i;
                let mut a = 0;
                while p < src.len() {
                    match src[p] {
                        '[' => {
                            a += 1;
                        }
                        ']' => {
                            a -= 1;
                            if a == 0 {
                                bf_brackets.insert(i, p);
                                bf_brackets.insert(p, i);
                                break;
                            }
                        }
                        _ => (),
                    }
                    p += 1;
                }
            }
            _ => (),
        }
        i += 1;
    }
    return bf_brackets;
}
