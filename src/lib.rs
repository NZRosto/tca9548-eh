#![doc = include_str!("../README.md")]
#![no_std]

use interface::Interface;

mod interface;

/// An error that occurs when communicating with the TCA9548.
#[derive(Debug)]
pub enum Error<EI2C> {
    /// An error occurred on the I2C bus.
    Io(EI2C),
    /// The I2C bus is busy, ie used by another multiplexed bus at the same
    /// time.
    BusBusy,
}

impl<EI2C> embedded_hal::i2c::Error for Error<EI2C>
where
    EI2C: embedded_hal::i2c::Error,
{
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        match self {
            Self::Io(e) => e.kind(),
            Self::BusBusy => embedded_hal::i2c::ErrorKind::Other,
        }
    }
}

/// A driver for the TCA9548 I2C bus multiplexer.
pub struct Tca9548<I2C> {
    interface: Interface<I2C>,
}

impl<I2C, EI2C> Tca9548<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = EI2C>,
{
    /// Create a new instance of the TCA9548 driver.
    ///
    /// # Errors
    /// Returns `Err` if the I2C bus is not responding.
    pub fn new(mut i2c: I2C, address: u8) -> Result<Self, EI2C> {
        // Select none of the I2C buses.
        i2c.write(address, &[0x00])?;

        Ok(Self {
            interface: Interface::new(spin::Mutex::new(i2c), address),
        })
    }

    /// Split the multiplexer into individual busses. This allows you to
    /// use each bus independently. A mutable reference is used to ensure
    /// multiple sets of busses cannot exist at the same time.
    pub fn split(&mut self) -> Busses<'_, I2C> {
        Busses {
            bus0: MultiplexedBus {
                interface: &self.interface,
            },
            bus1: MultiplexedBus {
                interface: &self.interface,
            },
            bus2: MultiplexedBus {
                interface: &self.interface,
            },
            bus3: MultiplexedBus {
                interface: &self.interface,
            },
            bus4: MultiplexedBus {
                interface: &self.interface,
            },
            bus5: MultiplexedBus {
                interface: &self.interface,
            },
            bus6: MultiplexedBus {
                interface: &self.interface,
            },
            bus7: MultiplexedBus {
                interface: &self.interface,
            },
        }
    }
}

/// The busses on the TCA9548.
pub struct Busses<'a, I2C> {
    /// Bus 0.
    pub bus0: MultiplexedBus<'a, I2C, 0>,
    /// Bus 1.
    pub bus1: MultiplexedBus<'a, I2C, 1>,
    /// Bus 2.
    pub bus2: MultiplexedBus<'a, I2C, 2>,
    /// Bus 3.
    pub bus3: MultiplexedBus<'a, I2C, 3>,
    /// Bus 4.
    pub bus4: MultiplexedBus<'a, I2C, 4>,
    /// Bus 5.
    pub bus5: MultiplexedBus<'a, I2C, 5>,
    /// Bus 6.
    pub bus6: MultiplexedBus<'a, I2C, 6>,
    /// Bus 7.
    pub bus7: MultiplexedBus<'a, I2C, 7>,
}

/// A multiplexed I2C bus. Using this bus will briefly enable the selected
/// channel on the TCA9548, then disable it again after the transaction is
/// complete.
pub struct MultiplexedBus<'a, I2C, const NUM: u8> {
    interface: &'a Interface<I2C>,
}

impl<'a, I2C, E, const NUM: u8> embedded_hal::i2c::ErrorType for MultiplexedBus<'a, I2C, NUM>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: embedded_hal::i2c::Error,
{
    type Error = Error<E>;
}

impl<'a, I2C, E, const NUM: u8> embedded_hal::i2c::I2c for MultiplexedBus<'a, I2C, NUM>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: embedded_hal::i2c::Error,
{
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Error<E>> {
        self.interface
            .enable_to_run::<NUM>(|i2c| i2c.transaction(address, operations))
    }
}
