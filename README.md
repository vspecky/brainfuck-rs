# Brainfuck.rs
An interpreter for the Brainfuck esoteric language written in Rust. What is brainfuck? Check out the wiki [here](https://esolangs.org/wiki/Brainfuck).

## About the Interpreter
- The interpreter allows for the standard memory size of 30,000 cells. So it won't be able to run programs that require more than that amount of cells.  
- The highest value a cell can store is 4,294,967,295 (Max unsigned 32-bit integer) and the lowest value is 0.  
- The interpreter allows for a maximum of 32,767 nested loops (Don't know why the hell would you require that many nested loops but that's the max).  
- There is no 'comment syntax', feel free to comment anything in your code (Avoid using any brainfuck command chars for this). The interpreter skips over any character that's not a valid brainfuck command.  
- Due to the way the Rust Standard Library handles stdin input, all user-input commands in your code (',') will run before anything is printed to stdout. (untested)  
- In case of any error in your brainfuck code, the interpreter will inform you about the nature of the error as well as the line and column numbers of the character at which the error occured.

## Usage
With Cargo :-
```
cargo run path/to/file
```
With binary/exe
```
./binary path/to/file
```