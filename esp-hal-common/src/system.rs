//! System
//!
//! The SYSTEM/DPORT peripheral needs to be split into several logical parts.
//!
//! Example
//! ```no_run
//! let peripherals = Peripherals::take().unwrap();
//! let system = peripherals.SYSTEM.split();
//! let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
//! ```
#[cfg(not(feature = "esp32"))]
type SystemPeripheral = crate::pac::SYSTEM;
#[cfg(feature = "esp32")]
type SystemPeripheral = crate::pac::DPORT;

/// Peripherals which can be enabled via [PeripheralClockControl]
pub enum Peripheral {
    Spi2,
    Spi3,
    I2cExt0,
    #[cfg(not(feature = "esp32c3"))]
    I2cExt1,
    Rmt,
}

/// Controls the enablement of peripheral clocks.
pub struct PeripheralClockControl {
    _private: (),
}

impl PeripheralClockControl {
    /// Enables and resets the given peripheral
    pub fn enable(&mut self, peripheral: Peripheral) {
        let system = unsafe { &*SystemPeripheral::PTR };

        #[cfg(not(feature = "esp32"))]
        let (perip_clk_en0, perip_rst_en0) = { (&system.perip_clk_en0, &system.perip_rst_en0) };
        #[cfg(feature = "esp32")]
        let (perip_clk_en0, perip_rst_en0) = { (&system.perip_clk_en, &system.perip_rst_en) };

        match peripheral {
            Peripheral::Spi2 => {
                perip_clk_en0.modify(|_, w| w.spi2_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.spi2_rst().clear_bit());
            }
            Peripheral::Spi3 => {
                perip_clk_en0.modify(|_, w| w.spi3_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.spi3_rst().clear_bit());
            }
            #[cfg(feature = "esp32")]
            Peripheral::I2cExt0 => {
                perip_clk_en0.modify(|_, w| w.i2c0_ext0_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.i2c0_ext0_rst().clear_bit());
            }
            #[cfg(not(feature = "esp32"))]
            Peripheral::I2cExt0 => {
                perip_clk_en0.modify(|_, w| w.i2c_ext0_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.i2c_ext0_rst().clear_bit());
            }
            #[cfg(not(feature = "esp32c3"))]
            Peripheral::I2cExt1 => {
                perip_clk_en0.modify(|_, w| w.i2c_ext1_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.i2c_ext1_rst().clear_bit());
            }
            Peripheral::Rmt => {
                perip_clk_en0.modify(|_, w| w.rmt_clk_en().set_bit());
                perip_rst_en0.modify(|_, w| w.rmt_rst().clear_bit());
            }
        }
    }
}

/// Controls the configuration of the chip's clocks.
pub struct SystemClockControl {
    _private: (),
}

/// The SYSTEM/DPORT splitted into it's different logical parts.
pub struct SystemParts {
    _private: (),
    pub peripheral_clock_control: PeripheralClockControl,
    pub clock_control: SystemClockControl,
}

/// Extension trait to split a SYSTEM/DPORT peripheral in independent logical
/// parts
pub trait SystemExt {
    type Parts;

    /// Splits the SYSTEM/DPORT peripheral into it's parts.
    fn split(self) -> Self::Parts;
}

impl SystemExt for SystemPeripheral {
    type Parts = SystemParts;

    fn split(self) -> Self::Parts {
        Self::Parts {
            _private: (),
            peripheral_clock_control: PeripheralClockControl { _private: () },
            clock_control: SystemClockControl { _private: () },
        }
    }
}
