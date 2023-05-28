use std::io;
use std::io::Stdin;

pub trait SystemOperations {
    fn request_input(&mut self) {}
    fn read_line(&self, buffer: &mut String);
}

pub struct SystemOperationsImpl {
    stdin: Option<Stdin>
}

impl SystemOperationsImpl {
    pub fn new() -> SystemOperationsImpl {
        SystemOperationsImpl {
            stdin: None
        }
    }
}

impl SystemOperations for SystemOperationsImpl {

    fn request_input(&mut self) {
        self.stdin = Some(io::stdin());
    }

    fn read_line(&self, buffer: &mut String) {
        match &self.stdin {
            Some(stdin) => {
                stdin.read_line(buffer).expect("Unable to read line from user");
            }
            None => {}
        }
    }
}
