# rc

The Rustic C Compiler is an incremental compiler for C99 in Rust.

## What

This is a project to learn [_rust_](https://rust-lang.com) and is currently a work in progress.
The compiler is built incrementally following the ideas in [Ghouloum - Incremental Approach to Compiler Construction](http://scheme2006.cs.uchicago.edu/11-ghuloum.pdf), while borrowing some ideas from other places.

## Why

A Compiler might seem like a complex piece of code the essence of which is simple in practice
the theory behind it not so much, it is also a great way to learn new languages as it
requires several features of your favorite _host_ language (Rust in this case).

Compilers are essentially a bunch of sequential transformations on data ('code') to turn it
into machine code, there are four main steps to write one :

1. Lexing : Essentially iterate over the code (represented by a _string_) while transforming
   character sequences into a list of tokens (sum types like enums are how you implement tokens see [src/token/mod.rs](token.rs)).

2. Parsing : Given a list of tokens build an abstract syntax tree (this can be done using LR/Pratt/Recursive Descent parsing)

3. Code Generation : Given an abstract syntax tree walk over the nodes (expressions or statements) and emit assembly, while it might seem
   difficult this step is essentially just a translation.

4. Bootstrapping : Assembly is great but to run the programs on your OS you will need to assemble and link your emitted assembly into a format
   understandable by your OS (_ELF_ for Linux and _PE_ for Windows or _Mach-O_ for OSX).

For the last two steps you can even write a register or stack based virtual machine with a small instruction set to simulate the silicon part.
Bootstrapping is platform specific so it might be easier to use gcc for this part.

Code that runs as native executables on your OS is essentially organized in a binary file format like _ELF_ or _PE_
the assembly you will generate will be translated into raw opcodes and organized into sections, with a main header that tells the OS how to execute the file.

You can learn more about this part in an OS course or book such as [OS in Three Easy Pieces](https://pages.cs.wisc.edu/~remzi/OSTEP/)

## When

This is essentially a work in progress, with extra documentation to explain the difficult steps.

## License

This work is licensed under the MIT License, see [LICENSE](LICENSE) for more information.
