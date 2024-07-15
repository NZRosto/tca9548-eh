use crate::Error;

pub(crate) struct Interface<I2C> {
    i2c: spin::Mutex<I2C>,
    address: u8,
}

impl<I2C, E> Interface<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    pub(crate) fn new(i2c: spin::Mutex<I2C>, address: u8) -> Self {
        Self { i2c, address }
    }

    /// Enable the given bus number to run a closure, then disable it again.
    pub(crate) fn enable_to_run<const NUM: u8>(
        &self,
        f: impl FnOnce(&mut I2C) -> Result<(), E>,
    ) -> Result<(), Error<E>> {
        let mut bus = self.i2c.try_lock().ok_or(Error::BusBusy)?;

        // Enable the bus.
        bus.write(self.address, &[1_u8 << NUM]).map_err(Error::Io)?;

        f(&mut bus).map_err(Error::Io)?;

        // Disable the bus.
        bus.write(self.address, &[0_u8]).map_err(Error::Io)?;

        Ok(())
    }
}
