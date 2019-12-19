use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug)]
pub struct ICError {
    details: String
}

impl ICError {
    fn new(msg: &str) -> ICError {
        ICError { details: msg.to_string() }
    }
}

impl fmt::Display for ICError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ICError {
    fn description(&self) -> &str {
        &self.details
    }
}

type ICResult<T> = std::result::Result<T, ICError>;

enum Instr {
    HALT,
    ADD { op1_reg: usize, op2_reg: usize, res_reg: usize },
    MULT { op1_reg: usize, op2_reg: usize, res_reg: usize },
}


pub struct Computer {
    program: Vec<u32>,
    memory: Vec<u32>,
    instr_ptr: usize
}

impl Computer {

    pub fn new(program: Vec<u32>) -> Computer {
        Computer {
            memory: program.clone(),
            program: program,
            instr_ptr: 0
        }
    }

    pub fn from_file(file_path: &str) -> std::io::Result<Computer> {
        let program: Vec<u32> = fs::read_to_string(file_path)?
            .trim()
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        Ok(Computer::new(program))
    }

    pub fn reset(&mut self) {
        self.instr_ptr = 0;
        self.memory = self.program.clone();
    }

    pub fn run(&mut self, noun: u32, verb: u32) -> ICResult<u32> {
        self.mem_set(1, noun)?;
        self.mem_set(2, verb)?;

        loop {
            match self.next_instr()? {
                Instr::ADD { op1_reg, op2_reg, res_reg } => {
                    let op1 = self.mem_read(op1_reg)?;
                    let op2 = self.mem_read(op2_reg)?;
                    self.mem_set(res_reg, op1 + op2)?;
                },
                Instr::MULT { op1_reg, op2_reg, res_reg } => {
                    let op1 = self.mem_read(op1_reg)?;
                    let op2 = self.mem_read(op2_reg)?;
                    self.mem_set(res_reg, op1 * op2)?;
                },
                Instr::HALT => break
            }
        }

        Ok(self.mem_read(0)?)
    }

    fn next_instr(&mut self) -> ICResult<Instr> {
        let instr = match self.mem_read(self.instr_ptr)? {
            99 => Instr::HALT,
            val => {
                let op1_reg = self.mem_read(self.instr_ptr + 1)? as usize;
                let op2_reg = self.mem_read(self.instr_ptr + 2)? as usize;
                let res_reg = self.mem_read(self.instr_ptr + 3)? as usize;
                self.instr_ptr += 4;
                match val {
                    1 => Instr::ADD { op1_reg, op2_reg, res_reg },
                    2 => Instr::MULT { op1_reg, op2_reg, res_reg },
                    _ => panic!("Unknown op code found.")
                }
            }
        };

        Ok(instr)
    }

    fn mem_set(&mut self, index: usize, val: u32) -> ICResult<()> {
        if index < self.memory.len() {
            self.memory[index] = val;
            Ok(())
        } else {
            Err(ICError::new("Memory index out of bounds"))
        }
    }

    fn mem_read(&self, index: usize) -> ICResult<u32> {
        if index < self.memory.len() {
            Ok(self.memory[index])
        } else {
            Err(ICError::new("Memory index out of bounds"))
        }
    }
}

