# Lisprs, A Lisp Interpreter In Rust

This is a Lisp interpreter implemented using Rust.

### Overview

This is an exercise in learning more about software architecture in Rust.
The project is split into two separate crates. `lisp-lib` is a library project
that will contain specific logic regarding the implementation of the Lisp
interpreter. `lisprs` is a binary application that contains the interpreter
itself.

### Motivation and Background

Part of the motivation behind this project is learning more about application
design patterns in Rust. Implementing a simple Lisp interpeter seemed like
a somewhat nontrivial task to accomplish that would provide me some space to
experiment more with Rust.


### References

This project will follow, at a very high-level, the dragon book in the design
of the interpreter.

### Todo

*  Basic Interpreter
  *  Read lines in from stdin
  *  Print output back to stdout
  *  Accept some basic command keywords (quit, help)
  *  Debug log (?)
*  Lisp Library
  *  Lexer
  *  AST class
  *  ...
