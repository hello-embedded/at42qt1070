#![no_std]

use embedded_hal::blocking::i2c;

// This will just contain all our addresses and constants related to the chip
mod chip {
    pub const I2C: u8 = 0x1B << 1;
    pub const ID: u8 = 0x2E;
    pub const ID_ADDR: u8 = 0;
}

#[derive(Clone, Copy, Debug)]
pub enum Error<I2cError> {
    I2cError(I2cError),
    IdMismatch(u8)
}
impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2cError(error)
    }
}

pub struct Driver<I2C> {
    i2c: I2C,
}

impl<I2C, I2cError> Driver<I2C>
where
    I2C: i2c::WriteRead<Error = I2cError>,
{
    pub fn new(i2c: I2C) -> Result<Driver<I2C>, Error<I2cError>> {
        let mut driver = Driver { i2c: i2c };
        let id = driver.get_id()?;
        if id != chip::ID {
            return Err(Error::IdMismatch(id));
        }
        Ok(driver)
    }

    fn get_id(&mut self) -> Result<u8, Error<I2cError>> {
        let mut buffer = [0u8; 1];
        self.i2c
            .write_read(chip::I2C, &[chip::ID_ADDR], &mut buffer)?;
        Ok(buffer[0])
    }
}
