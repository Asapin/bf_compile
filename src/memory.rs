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

    pub fn next_cell(&mut self) -> Result<(), Error> {
        self.pointer += 1;
        if self.pointer == self.data.len() {
            self.data.resize(self.data.len() * 2, 0);
        }

        Ok(())
    }

    pub fn prev_cell(&mut self) -> Result<(), Error> {
        if self.pointer == 0 {
            return Err(Error::new(
                ErrorKind::AddrNotAvailable,
                "Can't point to an address less than 0",
            ));
        }

        self.pointer -= 1;
        Ok(())
    }

    pub fn inc_value(&mut self) -> Result<(), Error> {
        self.data[self.pointer] += 1;
        Ok(())
    }

    pub fn dec_value(&mut self) -> Result<(), Error> {
        self.data[self.pointer] -= 1;
        Ok(())
    }

    pub fn write_value(&mut self, value: u8) -> Result<(), Error> {
        self.data[self.pointer] = value;
        Ok(())
    }

    pub fn read_value(&self) -> Result<u8, Error> {
        Ok(self.data[self.pointer])
    }
}
