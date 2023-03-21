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
    let bf_instructions: Vec<char> = "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++++++++++++++++++++++++++++++.>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>+++++++++++++++++++++++++++++++++.>".chars().collect();
    let mut bf_instruction_ptr: usize = 0;

    let mut bf_memory: Vec<i32> = vec![0; 32];
    let mut bf_memory_ptr: usize = 0;

    let mut loop_start: Vec<usize> = Vec::new();
    while bf_instruction_ptr < bf_instructions.len() {
        match bf_instructions[bf_instruction_ptr] {
            '>' => bf_memory_ptr += 1,
            '<' => bf_memory_ptr -= 1,
            '+' => bf_memory[bf_memory_ptr] = bf_memory[bf_memory_ptr] + 1,
            '-' => bf_memory[bf_memory_ptr] = bf_memory[bf_memory_ptr] - 1,
            '.' => {
                let printed_char = std::char::from_u32(bf_memory[bf_memory_ptr] as u32).unwrap();
                print!("{}", printed_char)
            }
            ',' => {
                print!("input char>");
                io::stdout().flush().unwrap();
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).unwrap();
                bf_memory[bf_memory_ptr] = (buf.chars().next().unwrap() as u32) as i32;
            }
            '[' => {
                if bf_memory[bf_memory_ptr] != 0 {
                    loop_start.push(bf_instruction_ptr);
                }
            }
            ']' => {
                if bf_memory[bf_memory_ptr] == 0 {
                    bf_instruction_ptr = loop_start.pop().unwrap();
                }
            }
            _ => {
                print!("\n<{}> |", bf_memory_ptr);
                for x in &bf_memory {
                    print!(" {} |", x)
                }
                println!();
            }
        }
        bf_instruction_ptr += 1;
    }
}
