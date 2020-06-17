use std::io::{Error, ErrorKind};

pub struct Memory {
    data: Vec<u8>,
    pointer: usize,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: vec![0; 30_000],
            pointer: 0,
        }
    }

    pub fn inc_mem_pointer_by(&mut self, val: usize) -> Result<(), Error> {
        self.pointer += val;
        if self.pointer >= self.data.len() {
            self.data.resize(self.pointer + 1, 0);
        }

        Ok(())
    }

    pub fn inc_mem_pointer_until_zero_value(&mut self, step: usize) -> Result<(), Error> {
        while self.pointer < self.data.len() && self.data[self.pointer] != 0 {
            self.pointer += step;
        }

        if self.pointer >= self.data.len() {
            self.data.resize(self.pointer + 1, 0);
        }

        Ok(())
    }

    pub fn dec_mem_pointer_by(&mut self, val: usize) -> Result<(), Error> {
        if val > self.pointer {
            return Err(Error::new(
                ErrorKind::AddrNotAvailable,
                "Can't point to an address less than 0",
            ));
        }

        self.pointer -= val;
        Ok(())
    }

    pub fn dec_mem_pointer_until_zero_value(&mut self, step: usize) -> Result<(), Error> {
        while self.pointer >= step && self.data[self.pointer] != 0 {
            self.pointer -= step;
        }

        if self.data[self.pointer] == 0 {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                "Couldn't find address with 0 value",
            ))
        }
    }

    pub fn inc_value_by(&mut self, val: u8) -> Result<(), Error> {
        if self.pointer < self.data.len() {
            self.data[self.pointer] += val;
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Tried to access address greater than available",
            ))
        }
    }

    pub fn dec_value_by(&mut self, val: u8) -> Result<(), Error> {
        if self.pointer < self.data.len() {
            self.data[self.pointer] -= val;
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Tried to access address greater than available",
            ))
        }
    }

    pub fn write_value(&mut self, val: u8) -> Result<(), Error> {
        if self.pointer < self.data.len() {
            self.data[self.pointer] = val;
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Tried to access address greater than available",
            ))
        }
    }

    pub fn read_value(&self) -> Result<u8, Error> {
        Ok(self.data[self.pointer])
    }
}
