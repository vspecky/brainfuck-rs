use std::vec::Vec;
use std::io::Read;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ./interpreter path/to/code/file");
        return;
    }

    let path = Path::new(&args[1]);

    match path.metadata() {
        Ok(meta) => {
            if !meta.is_file() {
                eprintln!("Error: Target is not a file.");
                return;
            }
        }

        Err(_) => {
            eprintln!("Error: Path does not exist.");
            return;
        }
    }

    let code = std::fs::read_to_string(&args[1]).expect("Error: Couldn't read file");

    Brainfuck::new(code).run();
}

// Main Brainfuck Struct
struct Brainfuck {
    mem: [u32; 30000],  // Memory. This interpreter follows the standard of 30k memory cells
    mem_ptr: usize,     // Memory Pointer
    prog: Vec<char>,    // Program file split into chars for easy reading
    pc: u32,            // Program Counter
    stack: Vec<u32>,    // Stack (for loops)
    sp: i16,            // Stack Pointer
    lin: u32,           // Line number of the current instruction (for debugging)
    col: u32,           // Column number of the current instruction (for debugging)
}

impl Brainfuck {
    // Get a new instance of Brainfuck
    fn new(prog_str: String) -> Self {
        let out = Self {
            mem: [0; 30000],
            mem_ptr: 0,
            prog: prog_str.chars().collect(),
            pc: 0,
            stack: Vec::new(),
            sp: -1,
            lin: 1,
            col: 1
        };

        out
    }

    // Push the current program counter, line value and col value to the stack.
    // The stack pointer is only incremented by one and we can peek at any
    // (pc, lin, col) triple by multiplying the required triple's apparent
    // index by 3 and adding 0, 1, or 2 to get the pc, lin or col respectively.
    fn stack_push(&mut self) -> Result<(), &'static str> {
        if self.sp == i16::max_value() {
            Err("Nested loop limit reached.")
        } else {
            self.sp += 1;

            self.stack.push(self.pc);
            self.stack.push(self.lin);
            self.stack.push(self.col);

            Ok(())
        }
    }

    // Pop the topmost (pc, lin, col) triple and return it
    fn stack_pop(&mut self) -> Result<(u32, u32, u32), &'static str> {
        if self.sp < 0 {
            return Err("Unpaired ']'.");
        }

        let col = self.stack.pop().unwrap();
        let lin = self.stack.pop().unwrap();
        let pc = self.stack.pop().unwrap();
        self.sp -= 1;

        Ok((pc, lin, col))
    }

    // Peek at the topmost (pc, lin, col) triple without popping it from
    // the stack
    fn stack_peek(&mut self) -> Result<(u32, u32, u32), &'static str> {
        if self.sp >= 0 {
            let pc = self.stack[(self.sp * 3) as usize];
            let lin = self.stack[(self.sp * 3 + 1) as usize];
            let col = self.stack[(self.sp * 3 + 2) as usize];

            Ok((pc, lin, col))
        } else {
            Err("Tried to peek empty stack")
        }
    }

    // Return the value of the cell to which the memory pointer is
    // currently pointing to
    fn read_cell(&self) -> u32 {
        self.mem[self.mem_ptr]
    }

    // Write the supplied value to the current cell pointed to by
    // the memory pointer
    fn write_to_cell(&mut self, val: u32) {
        self.mem[self.mem_ptr] = val;
    }

    // Skip over all whitespace and non-brainfuck characters while
    // simultaneously updating the program counter, line and column numbers
    // and return the next instruction or return None if no instruction is found
    fn next_instruction(&mut self) -> Option<char> {
        while (self.pc as usize) < self.prog.len() {
            match self.prog[self.pc as usize] {
                '<' | '>' | '+' | '-' |
                '.' | ',' | '[' | ']' => {
                    let ch = self.prog[self.pc as usize];
                    self.pc += 1;
                    self.col += 1;
                    return Some(ch);
                }

                '\n' => {
                    self.pc += 1;
                    self.lin += 1;
                    self.col = 1;
                }

                _ => {
                    self.pc += 1;
                    self.col += 1;
                }
            }
        }

        None
    }

    // Check if the '[' symbol at the current program counter
    // location has a corresponding ']'
    fn check_ending_bracket(&mut self) -> Option<u32> {
        let mut i = self.pc as usize;

        let mut opening_brackets = 0;
        while i < self.prog.len() {
            let c = self.prog[i];

            if c == ']' {
                if opening_brackets == 0 {
                    return Some(i as u32);
                } else {
                    opening_brackets -= 1;
                }

            } else if c == '[' {
                opening_brackets += 1;
            }

            i += 1;
        }

        None
    }

    // Interpret the code
    fn run(&mut self) {
        loop {
            let ins = match self.next_instruction() {
                Some(ch) => ch,
                None => break
            };

            let status: Result<(), &'static str> = match ins {
                // Move the memory pointer to the left
                '<' => {
                    if self.mem_ptr == 0 {
                        Err("Tried to access memory out of range (underflow)")
                    } else {
                        self.mem_ptr -= 1;
                        Ok(())
                    }
                }

                // Move the memory pointer to the right
                '>' => {
                    if self.mem_ptr == 29999 {
                        Err("Tried to access memory out of range (overflow)")
                    } else {
                        self.mem_ptr += 1;
                        Ok(())
                    }
                }

                // Increment the value of the cell at the memory pointer
                '+' => {
                    if self.read_cell() == u32::max_value() {
                        Err("Exceeded max cell value")
                    } else {
                        self.write_to_cell(self.read_cell() + 1);
                        Ok(())
                    }
                }

                // Decrement the value of the cell at the memory pointer
                '-' => {
                    if self.read_cell() == 0 {
                        Err("Cells cannot have negative values")
                    } else {
                        self.write_to_cell(self.read_cell() - 1);
                        Ok(())
                    }
                }

                // Signifies the start of a loop
                // If the value of the cell at the memory pointer is 0,
                // skip to the corresponding ']'
                '[' => {
                    if let Some(addr) = self.check_ending_bracket() {
                        if self.read_cell() == 0 {
                            self.pc = addr + 1;
                            Ok(())
                        } else {
                            self.stack_push()
                        }
                    } else {
                        Err("Loop not closed")
                    }
                }

                // Signifies the end of a loop
                // If the value of the cell at the memory pointer is
                // non-zero, go back to the start of the loop
                ']' => {
                    if let Err(_) = self.stack_peek() {
                        Err("Obsolete loop close bracket")
                    } else {
                        if self.read_cell() != 0 {
                            let (pc, lin, col) = self.stack_peek().unwrap();
                            self.pc = pc;
                            self.lin = lin;
                            self.col = col;
                        } else {
                            self.stack_pop().unwrap();
                        }

                        Ok(())
                    }
                }

                // Converts the value of the current cell to ascii and
                // prints it
                '.' => {
                    if let Some(ch) = std::char::from_u32(self.read_cell()) {
                        print!("{}", ch);
                        Ok(())
                    } else {
                        Err("Could not print character")
                    }
                }

                // Takes in a single character as input and stores its value
                // in the current cell
                ',' => {
                    let mut ch = [0];
                    std::io::stdin().read_exact(&mut ch).expect("Couldn't read from std");
                    while ch[0] == '\n' as u8 {
                        std::io::stdin().read_exact(&mut ch).expect("Couldn't read from std");
                    }

                    self.write_to_cell(ch[0].into());
                    Ok(())
                }

                _ => Err("Unexpected instruction")
            };

            if let Err(msg) = status {
                println!("\nError: {} ({}: {})", msg, self.lin, self.col);
                break;
            }
        }
    }
}