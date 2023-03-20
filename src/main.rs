// Character 	Meaning
// > 	Increment the data pointer (to point to the next cell to the right).
// < 	Decrement the data pointer (to point to the next cell to the left).
// + 	Increment (increase by one) the byte at the data pointer.
// - 	Decrement (decrease by one) the byte at the data pointer.
// . 	Output the byte at the data pointer.
// , 	Accept one byte of input, storing its value in the byte at the data pointer.
// [ 	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
// ] 	If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.

fn main() {
    let mut bf_p: usize = 0;
    let mut bf_v: Vec<i32> = vec![0; 32];

    // let bf_i = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".chars();
    let bf_i = "+++++>>+++>>>,..>+++..]".chars();
    for c in bf_i {
        match c {
            '>' => bf_p += 1,
            '<' => bf_p -= 1,
            '+' => bf_v[bf_p] = bf_v[bf_p] + 1,
            '-' => bf_v[bf_p] = bf_v[bf_p] - 1,
            '.' => {
                let p_c = std::char::from_u32(bf_v[bf_p] as u32).unwrap();
                print!("{}", p_c)
            }
            ',' => {
                println!("\ninput char>");
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).unwrap();
                bf_v[bf_p] = (buf.chars().next().unwrap() as u32) as i32;
            }
            _ => {
                print!("\n<{}> |", bf_p);
                for x in &bf_v {
                    print!(" {} |", x)
                }
                println!();
            }
        }
    }

}
