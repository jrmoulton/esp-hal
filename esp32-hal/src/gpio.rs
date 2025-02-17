//! General Purpose I/Os
//!
//! To get access to the pins, you first need to convert them into a HAL
//! designed struct from the pac struct `GPIO` and `IO_MUX` using `IO::new`.
//!
//! ```no_run
//! let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
//! let mut led = io.pins.gpio5.into_push_pull_output();
//! ```

use esp_hal_common::gpio::{types::*, *};

gpio! {
    Function2,
    DualCore,

    Gpio0:  ( gpio0,  0,  gpio0, IO, RTC, Bank0, touch_pad1), (EMAC_TX_CLK: Function5), (CLK_OUT1: Function1),
    Gpio1:  ( gpio1,  1,  gpio1, IO,   0, Bank0, None), (EMAC_RXD2: Function5), (U0TXD: Function1, CLK_OUT3: Function1),
    Gpio2:  ( gpio2,  2,  gpio2, IO, RTC, Bank0, touch_pad2), (HSPIWP: Function1, HS2_DATA0: Function3, SD_DATA0: Function4), (HS2_DATA0: Function3, SD_DATA0: Function4),
    Gpio3:  ( gpio3,  3,  gpio3, IO,   0, Bank0, None), (U0RXD: Function0), (CLK_OUT2: Function1),
    Gpio4:  ( gpio4,  4,  gpio4, IO, RTC, Bank0, touch_pad0), (HSPIHD: Function1, HS2_DATA1: Function3, SD_DATA1: Function4, EMAC_TX_ER: Function5), (HS2_DATA1: Function3, SD_DATA1: Function4),
    Gpio5:  ( gpio5,  5,  gpio5, IO,   0, Bank0, None), (VSPICS0: Function1, HS1_DATA6: Function3, EMAC_RX_CLK: Function5), (HS1_DATA6: Function3),
    Gpio6:  ( gpio6,  6,  gpio6, IO,   0, Bank0, None), (U1CTS: Function4), (SD_CLK: Function0, SPICLK: Function1, HS1_CLK: Function3),
    Gpio7:  ( gpio7,  7,  gpio7, IO,   0, Bank0, None), (SD_DATA0: Function0, SPIQ: Function1, HS1_DATA0: Function3), (SD_DATA0: Function0, SPIQ: Function1, HS1_DATA0: Function3, U2RTS: Function4),
    Gpio8:  ( gpio8,  8,  gpio8, IO,   0, Bank0, None), (SD_DATA1: Function0, SPID: Function1, HS1_DATA1: Function3, U2CTS: Function4), (SD_DATA1: Function0, SPID: Function1, HS1_DATA1: Function3),
    Gpio9:  ( gpio9,  9,  gpio9, IO,   0, Bank0, None), (SD_DATA2: Function0, SPIHD: Function1, HS1_DATA2: Function3, U1RXD: Function4), (SD_DATA2: Function0, SPIHD: Function1, HS1_DATA2: Function3),
    Gpio10: (gpio10, 10, gpio10, IO,   0, Bank0, None), (SD_DATA3: Function0, SPIWP: Function1, HS1_DATA3: Function3), (SD_DATA3: Function0, SPIWP: Function1, HS1_DATA3: Function3, U1TXD: Function4),
    Gpio11: (gpio11, 11, gpio11, IO,   0, Bank0, None), (SPICS0: Function1), (SD_CMD: Function0, SPICS0: Function1, HS1_CMD: Function3, U1RTS: Function4),
    Gpio12: (gpio12, 12, gpio12, IO, RTC, Bank0, touch_pad5), (MTDI: Function0, HSPIQ: Function1, HS2_DATA2: Function3, SD_DATA2: Function4), (HSPIQ: Function1, HS2_DATA2: Function3, SD_DATA2: Function4, EMAC_TXD3: Function5),
    Gpio13: (gpio13, 13, gpio13, IO, RTC, Bank0, touch_pad4), (MTCK: Function0, HSPID: Function1, HS2_DATA3: Function3, SD_DATA3: Function4), (HSPID: Function1, HS2_DATA3: Function3, SD_DATA3: Function4, EMAC_RX_ER: Function5),
    Gpio14: (gpio14, 14, gpio14, IO, RTC, Bank0, touch_pad6), (MTMS: Function0, HSPICLK: Function1), (HSPICLK: Function1, HS2_CLK: Function3, SD_CLK: Function4, EMAC_TXD2: Function5),
    Gpio15: (gpio15, 15, gpio15, IO, RTC, Bank0, touch_pad3), (HSPICS0: Function1, EMAC_RXD3: Function5), (MTDO: Function0, HSPICS0: Function1, HS2_CMD: Function3, SD_CMD: Function4),
    Gpio16: (gpio16, 16, gpio16, IO,   0, Bank0, None), (HS1_DATA4: Function3, U2RXD: Function4), (HS1_DATA4: Function3, EMAC_CLK_OUT: Function5),
    Gpio17: (gpio17, 17, gpio17, IO,   0, Bank0, None), (HS1_DATA5: Function3), (HS1_DATA5: Function3, U2TXD: Function4, EMAC_CLK_180: Function5),
    Gpio18: (gpio18, 18, gpio18, IO,   0, Bank0, None), (VSPICLK: Function1, HS1_DATA7: Function3), (VSPICLK: Function1, HS1_DATA7: Function3),
    Gpio19: (gpio19, 19, gpio19, IO,   0, Bank0, None), (VSPIQ: Function1, U0CTS: Function3), (VSPIQ: Function1, EMAC_TXD0: Function5),
    Gpio20: (gpio20, 20, gpio20, IO,   0, Bank0, None),
    Gpio21: (gpio21, 21, gpio21, IO,   0, Bank0, None), (VSPIHD: Function1), (VSPIHD: Function1, EMAC_TX_EN: Function5),

    Gpio22: (gpio22, 22, gpio22, IO, 0, Bank0, None), (VSPIWP: Function1), (VSPIWP: Function1, U0RTS: Function3, EMAC_TXD1: Function5),
    Gpio23: (gpio23, 23, gpio23, IO, 0, Bank0, None), (VSPID: Function1), (VSPID: Function1, HS1_STROBE: Function3),
    Gpio24: (gpio24, 24, gpio24, IO, 0, Bank0, None),
    Gpio25: (gpio25, 25, gpio25, IO, 0, Bank0, pad_dac1), (EMAC_RXD0: Function5), (),
    Gpio26: (gpio26, 26, gpio26, IO, 0, Bank0, pad_dac2), (EMAC_RXD1: Function5), (),
    Gpio27: (gpio27, 27, gpio27, IO, 0, Bank0, touch_pad7), (EMAC_RX_DV: Function5), (),

    Gpio32: (gpio32, 32, gpio32,    IO, 0, Bank1, xtal_32k_n),
    Gpio33: (gpio33, 33, gpio33,    IO, 0, Bank1, xtal_32k_p),
    Gpio34: (gpio34, 34, gpio34, Input, 0, Bank1, None),
    Gpio35: (gpio35, 35, gpio35, Input, 0, Bank1, None),
    Gpio36: (gpio36, 36, gpio36, Input, 0, Bank1, None),
    Gpio37: (gpio37, 37, gpio37, Input, 0, Bank1, None),
    Gpio38: (gpio38, 38, gpio38, Input, 0, Bank1, None),
    Gpio39: (gpio39, 39, gpio39, Input, 0, Bank1, None),
}

analog! {[
    Gpio36: (0,  sensor_pads,  sense1_hold, sense1_mux_sel, sense1_fun_sel, sense1_fun_ie, sense1_slp_ie, sense1_slp_sel ),
    Gpio37: (1,  sensor_pads,  sense2_hold, sense2_mux_sel, sense2_fun_sel, sense2_fun_ie, sense2_slp_ie, sense2_slp_sel ),
    Gpio38: (2,  sensor_pads,  sense3_hold, sense3_mux_sel, sense3_fun_sel, sense3_fun_ie, sense3_slp_ie, sense3_slp_sel ),
    Gpio39: (3,  sensor_pads,  sense4_hold, sense4_mux_sel, sense4_fun_sel, sense4_fun_ie, sense4_slp_ie, sense4_slp_sel ),
    Gpio34: (4,  adc_pad,      adc1_hold,   adc1_mux_sel,   adc1_fun_sel,   adc1_fun_ie,   adc1_slp_ie,   adc1_slp_sel   ),
    Gpio35: (5,  adc_pad,      adc2_hold,   adc2_mux_sel,   adc2_fun_sel,   adc1_fun_ie,   adc1_slp_ie,   adc1_slp_sel   ),
    Gpio25: (6,  pad_dac1,     pdac1_hold,  pdac1_mux_sel,  pdac1_fun_sel,  pdac1_fun_ie,  pdac1_slp_ie,  pdac1_slp_sel,  pdac1_rue, pdac1_rde, pdac1_drv, pdac1_slp_oe),
    Gpio26: (7,  pad_dac2,     pdac2_hold,  pdac2_mux_sel,  pdac2_fun_sel,  pdac2_fun_ie,  pdac2_slp_ie,  pdac2_slp_sel,  pdac2_rue, pdac2_rde, pdac2_drv, pdac2_slp_oe),
    Gpio33: (8,  xtal_32k_pad, x32n_hold,   x32n_mux_sel,   x32n_fun_sel,   x32n_fun_ie,   x32n_slp_ie,   x32n_slp_sel,   x32n_rue,  x32n_rde,  x32n_drv,  x32n_slp_oe),
    Gpio32: (9,  xtal_32k_pad, x32p_hold,   x32p_mux_sel,   x32p_fun_sel,   x32p_fun_ie,   x32p_slp_ie,   x32p_slp_sel,   x32p_rue,  x32p_rde,  x32p_drv,  x32p_slp_oe),
    Gpio4:  (10, touch_pad0,   hold,        mux_sel,        fun_sel,        fun_ie,        slp_ie,        slp_sel,        rue,       rde,       drv,       slp_oe),
    Gpio0:  (11, touch_pad1,   hold,        mux_sel,        fun_sel,        fun_ie,        slp_ie,        slp_sel,        rue,       rde,       drv,       slp_oe),
    Gpio2:  (12, touch_pad2,   hold,        mux_sel,        fun_sel,        fun_ie,        slp_ie,        slp_sel,        rue,       rde,       drv,       slp_oe),
    Gpio15: (13, touch_pad3,   hold,        mux_sel,        fun_sel,        fun_ie,        slp_ie,        slp_sel,        rue,       rde,       drv,       slp_oe),
    Gpio13: (14, touch_pad4,   hold,        mux_sel,        fun_sel,        fun_ie,        slp_ie,        slp_sel,        rue,       rde,       drv,       slp_oe),
    Gpio12: (15, touch_pad5,   hold,        mux_sel,        fun_sel,        fun_ie,        slp_ie,        slp_sel,        rue,       rde,       drv,       slp_oe),
    Gpio14: (16, touch_pad6,   hold,        mux_sel,        fun_sel,        fun_ie,        slp_ie,        slp_sel,        rue,       rde,       drv,       slp_oe),
    Gpio27: (17, touch_pad7,   hold,        mux_sel,        fun_sel,        fun_ie,        slp_ie,        slp_sel,        rue,       rde,       drv,       slp_oe),
]}
