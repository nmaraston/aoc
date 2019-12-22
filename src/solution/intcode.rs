use std::error::Error;
use std::fmt;

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

enum Param {
    POSITION(usize),
    IMMEDIATE(i32)
}

enum Instr {
    HALT,
    ADD(Param, Param, usize),
    MULT(Param, Param, usize),
    INPUT(usize),
    OUTPUT(Param),
}

pub struct Computer<'a> {
    program: Vec<i32>,
    memory: Vec<i32>,
    instr_ptr: usize,
    input_device: &'a dyn Fn() -> i32,
    output_device: &'a mut dyn FnMut(i32),
}

impl<'a> Computer<'a> {

    pub fn new(
        input_device: &'a dyn Fn() -> i32,
        output_device: &'a mut dyn FnMut(i32),
        program: Vec<i32>) -> Self {
        Self {
            memory: program.clone(),
            program: program,
            instr_ptr: 0,
            input_device: input_device,
            output_device: output_device,
        }
    }

    pub fn reset(&mut self) {
        self.instr_ptr = 0;
        self.memory = self.program.clone();
    }

    pub fn set_noun(&mut self, noun: i32) {
        self.mem_set(1, noun).unwrap();
    }

    pub fn set_verb(&mut self, verb: i32) {
        self.mem_set(2, verb).unwrap();
    }

    pub fn run(&mut self) -> ICResult<()> {
        loop {
            match self.next_instr()? {
                Instr::ADD(param1, param2, reg) => {
                    let arg1 = self.read_param_arg(param1)?;
                    let arg2 = self.read_param_arg(param2)?;
                    let res = arg1 + arg2;
                    self.mem_set(reg, res)?
                },
                Instr::MULT(param1, param2, reg) => {
                    let arg1 = self.read_param_arg(param1)?;
                    let arg2 = self.read_param_arg(param2)?;
                    let res = arg1 * arg2;
                    self.mem_set(reg, res)?
                },
                Instr::INPUT(reg) => {
                    let input = (self.input_device)();
                    self.mem_set(reg, input)?
                },
                Instr::OUTPUT(param) => {
                    let arg = self.read_param_arg(param)?;
                    (self.output_device)(arg);
                },
                Instr::HALT => break,
            }
        }
        
        Ok(())
    }

    fn next_instr(&mut self) -> ICResult<Instr> {
        let instr_code = self.mem_read(self.instr_ptr)?;
        let instr = match instr_code % 100 {
            1 => {
                let param1 = self.read_param(instr_code, 1)?; 
                let param2 = self.read_param(instr_code, 2)?; 
                let reg = self.mem_read(self.instr_ptr + 3)? as usize; 
                self.instr_ptr += 4;
                Instr::ADD(param1, param2, reg)
            },
            2 => {
                let param1 = self.read_param(instr_code, 1)?; 
                let param2 = self.read_param(instr_code, 2)?; 
                let reg = self.mem_read(self.instr_ptr + 3)? as usize; 
                self.instr_ptr += 4;
                Instr::MULT(param1, param2, reg)
            },
            3 => {
                let reg = self.mem_read(self.instr_ptr + 1)? as usize; 
                self.instr_ptr += 2;
                Instr::INPUT(reg)
            },
            4 => {
                let param = self.read_param(instr_code, 1)?; 
                self.instr_ptr += 2;
                Instr::OUTPUT(param)
            },
            99 => Instr::HALT,
            _ => panic!("Unknown op code found."),
        };

        Ok(instr)
    }

    fn read_param(&self, instr_code: i32, param_number: usize) -> ICResult<Param> {
        let mode = (instr_code / (100 * 10_i32.pow(param_number as u32 - 1))) % 10;
        let val = self.mem_read(self.instr_ptr + param_number)?;
        let param = match mode {
            0 => Param::POSITION(val as usize),
            1 => Param::IMMEDIATE(val),
            _ => panic!("Unknown param mode found."),
        };

        Ok(param)
    }

    fn read_param_arg(&self, param: Param) -> ICResult<i32> {
        let result = match param {
            Param::POSITION(val) => self.mem_read(val as usize)?,
            Param::IMMEDIATE(val) => val,
        };

        Ok(result)
    }

    fn mem_set(&mut self, index: usize, val: i32) -> ICResult<()> {
        if index < self.memory.len() {
            self.memory[index] = val;
            Ok(())
        } else {
            Err(ICError::new("Memory index out of bounds"))
        }
    }

    pub fn mem_read(&self, index: usize) -> ICResult<i32> {
        if index < self.memory.len() {
            Ok(self.memory[index])
        } else {
            Err(ICError::new("Memory index out of bounds"))
        }
    }
}

