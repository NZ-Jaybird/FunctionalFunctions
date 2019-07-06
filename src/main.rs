pub mod vm;
pub mod instruction;
pub mod repl;
pub mod assembler;

#[macro_use]
    extern crate nom;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
