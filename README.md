<h1 align="center">🧠 Brainfuck Interpreter</h1>
<h5 align="center">This is a simple Brainfuck interpreter written in Rust.</h5>

------

## Introduction

Brainfuck is an esoteric programming language created in 1993 by Urban Müller. It consists of only eight commands, each of which is represented by a single character. The language is designed to challenge and amuse programmers, rather than to be suitable for practical use.

## Grammar

To run the interpreter, simply compile the Rust code and run the resulting binary file. The interpreter takes a string of Brainfuck code as input and executes it.

The following is a list of the eight Brainfuck commands and their corresponding meanings:

| Character | Meaning                                                                                                                                                                   |
|-----------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| >         | Increment the data pointer (to point to the next cell to the right).                                                                                                       |
| <         | Decrement the data pointer (to point to the next cell to the left).                                                                                                        |
| +         | Increment (increase by one) the byte at the data pointer.                                                                                                                  |
| -         | Decrement (decrease by one) the byte at the data pointer.                                                                                                                  |
| .         | Output the byte at the data pointer.                                                                                                                                       |
| ,         | Accept one byte of input, storing its value in the byte at the data pointer.                                                                                                |
| [         | If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command. |
| ]         | If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command. |

## Implementation

The interpreter is implemented in Rust, and consists of a loop that reads each character of the input string in turn and executes the corresponding Brainfuck command.

The interpreter uses a vector to represent the Brainfuck memory, which is initialized to 32 cells. The data pointer is represented by an index into this vector. The interpreter also uses a vector to keep track of the locations of loop start commands, so that it can jump back to them when it encounters a loop end command.

## Example

Here is an example of a Brainfuck program that prints "Hello World!":

```brainfuck
++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++.>+.+++++++..+++.<<++.>+++++++++++++++.>.+++.------.--------.<<+.<.
```

When executed by the interpreter, this program outputs the following text:

```text
Hello World!
```

## License

Copyright &copy; 2023 [Core2002](https://github.com/Core2002)

Licensed under the MIT License. See LICENSE for details.
