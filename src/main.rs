// Character 	Meaning
// > 	Increment the data pointer (to point to the next cell to the right).
// < 	Decrement the data pointer (to point to the next cell to the left).
// + 	Increment (increase by one) the byte at the data pointer.
// - 	Decrement (decrease by one) the byte at the data pointer.
// . 	Output the byte at the data pointer.
// , 	Accept one byte of input, storing its value in the byte at the data pointer.
// [ 	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
// ] 	If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.

use std::io::{self, Write};

fn main() {
    let src = "++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++.>+.+++++++..+++.<<++.>+++++++++++++++.>.+++.------.--------.<<+.<.";
    let bf_instructions: Vec<char> = src.chars().collect();
    let mut bf_instruction_ptr: usize = 0;

    let mut bf_memory: Vec<i64> = vec![0; 32];
    let mut bf_memory_ptr: usize = 0;

    let bf_pp = find_parentheses_positions(src).unwrap();
    // dbg!(&bf_pp);
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
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).unwrap();
                bf_memory[bf_memory_ptr] = buf.chars().next().unwrap() as i64;
            }
            '[' => {
                if bf_memory[bf_memory_ptr] == 0 {
                    let mut flag = false;
                    for pp in &bf_pp {
                        if pp.0 == bf_instruction_ptr {
                            bf_instruction_ptr = pp.1;
                            flag = true;
                            break;
                        }
                    }
                    if !flag {
                        panic!("ERROR")
                    }
                }
            }
            ']' => {
                if bf_memory[bf_memory_ptr] != 0 {
                    let mut flag = false;
                    for pp in &bf_pp {
                        if pp.1 == bf_instruction_ptr {
                            bf_instruction_ptr = pp.0;
                            flag = true;
                            break;
                        }
                    }
                    if !flag {
                        panic!("ERROR")
                    }
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

fn find_parentheses_positions(s: &str) -> Option<Vec<(usize, usize)>> {
    let mut stack = vec![];
    let mut pairs = vec![];
    for (i, ch) in s.char_indices() {
        if ch == '[' {
            stack.push(i);
        } else if ch == ']' {
            if let Some(left_i) = stack.pop() {
                pairs.push((left_i, i));
            } else {
                return None;
            }
        }
    }
    if !stack.is_empty() {
        return None;
    }
    Some(pairs)
}
