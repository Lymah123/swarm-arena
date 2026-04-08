use std::io::Read;

fn main() {
    let code = r#"[<+>>>>>>>>++++++++++<<<<<<<-]>+++++[<+++++++++>-]+>>>>>>+[<<+++[>>[-<]<[>]<-]>>[>+>]<[<]>]>[[->>>>+<<<<]>>>+++>-]<[<<<<]<<<<<<<<+[->>>>>>>>>>>>[<+[->>>>+<<<<]>>>>]<<<<[>>>>>[<<<<+>>>>-]<<<<<-[<<++++++++++>>-]>>>[<<[<+<<+>>>-]<[>+<-]<++<<+>>>>>>-]<<[-]<<-<[->>+<-[>>>]>[[<+>-]>+>>]<<<<<]>[-]>+<<<-[>>+<<-]<]<<<<+>>>>>>>>[-]>[<<<+>>>-]<<++++++++++<[->>+<-[>>>]>[[<+>-]>+>>]<<<<<]>[-]>+>[<<+<+>>>-]<<<<+<+>>[-[-[-[-[-[-[-[-[-<->[-<+<->>]]]]]]]]]]<[+++++[<<<++++++++<++++++++>>>>-]<<<<+<->>>>[>+<<<+++++++++<->>>-]<<<<<[>>+<<-]+<[->-<]>[>>.<<<<[+.[-]]>>-]>[>>.<<-]>[-]>[-]>>>[>>[<<<<<<<<+>>>>>>>>-]<<-]]>>[-]<<<[-]<<<<<<<<]++++++++++."#;

    let mut memory = vec![0u8; 30000];
    let mut ptr = 0;
    let mut pc = 0;
    let code_bytes = code.as_bytes();

    while pc < code_bytes.len() {
        match code_bytes[pc] {
            b'>' => {
                ptr += 1;
                if ptr >= memory.len() {
                    ptr = memory.len() - 1;
                }
            }
            b'<' => {
                if ptr > 0 {
                    ptr -= 1;
                }
            }
            b'+' => {
                memory[ptr] = memory[ptr].wrapping_add(1);
            }
            b'-' => {
                memory[ptr] = memory[ptr].wrapping_sub(1);
            }
            b'.' => {
                print!("{}", memory[ptr] as char);
            }
            b',' => {
                let mut buf = [0; 1];
                if std::io::stdin().read_exact(&mut buf).is_ok() {
                    memory[ptr] = buf[0];
                }
            }
            b'[' => {
                if memory[ptr] == 0 {
                    let mut depth = 1;
                    pc += 1;
                    while depth > 0 && pc < code_bytes.len() {
                        if code_bytes[pc] == b'[' {
                            depth += 1;
                        } else if code_bytes[pc] == b']' {
                            depth -= 1;
                        }
                        if depth > 0 {
                            pc += 1;
                        }
                    }
                }
            }
            b']' => {
                if memory[ptr] != 0 {
                    let mut depth = 1;
                    pc -= 1;
                    while depth > 0 && pc > 0 {
                        if code_bytes[pc] == b']' {
                            depth += 1;
                        } else if code_bytes[pc] == b'[' {
                            depth -= 1;
                        }
                        if depth > 0 {
                            pc -= 1;
                        }
                    }
                }
            }
            _ => {}
        }
        pc += 1;
    }
    println!(); // newline at end
}
