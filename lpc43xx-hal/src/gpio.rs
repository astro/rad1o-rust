//! General Purpose Input / Output

use target::{CCU1, RGU, GPIO_PORT, gpio_port};
use hal::digital::{InputPin, OutputPin};
use core::marker::PhantomData;

/// Extension trait to split a GPIO_PORT peripheral in independent pins and registers
pub trait GpioExt {
    /// The parts to split the GPIO_PORT into
    type Parts;

    /// Splits the GPIO_PORT block into independent pins and registers
    fn split(self, ccu1: &mut CCU1, rgu: &mut RGU) -> Self::Parts;
}

/// Input mode (type state)
pub struct Input;

/// Output mode (type state)
pub struct Output;


#[inline(always)]
fn gpio_port() -> &'static gpio_port::RegisterBlock {
    unsafe { &*GPIO_PORT::ptr() }
}

macro_rules! gpio {
    [
        $($PXi:ident: ($pxi:ident, $port:expr, $pin:expr, $MODE:ty, $dirp:ident, $clrp:ident, $setp:ident),)+
    ] => {
        /// GPIO parts
        pub struct Parts {
            $(
                /// Pin
                pub $pxi: $PXi<$MODE>,
            )+
        }

        impl GpioExt for GPIO_PORT {
            type Parts = Parts;

            fn split(self, ccu1: &mut CCU1, rgu: &mut RGU) -> Parts {
                ccu1.clk_m4_gpio_cfg.write(
                    |w| w
                        .run().enabled()
                        .auto().enabled()
                );
                rgu.reset_ctrl0.write(
                    |w| w.gpio_rst().set_bit()
                );

                Parts {
                    $(
                        $pxi: $PXi { _mode: PhantomData },
                    )+
                }
            }
        }

        $(
            /// Pin
            pub struct $PXi<MODE> {
                _mode: PhantomData<MODE>,
            }

            impl<MODE> $PXi<MODE> {
                pub fn into_input(self) -> $PXi<Input> {
                    gpio_port().dir[$port].modify(
                        |_, w| w.$dirp().clear_bit()
                    );

                    $PXi { _mode: PhantomData }
                }

                pub fn into_output(self) -> $PXi<Output> {
                    gpio_port().dir[$port].modify(
                        |_, w| w.$dirp().set_bit()
                    );

                    $PXi { _mode: PhantomData }
                }
            }

            impl OutputPin for $PXi<Output> {
                fn set_high(&mut self) {
                    gpio_port().set[$port].write(
                        |w| w.$setp().set_bit()
                    );
                }

                fn set_low(&mut self) {
                    gpio_port().clr[$port].write(
                        |w| w.$clrp().set_bit()
                    );
                }
            }

            impl InputPin for $PXi<Input> {
                fn is_high(&self) -> bool {
                    gpio_port().b[(32 * $port) + $pin].read()
                        .pbyte().bit()
                }

                fn is_low(&self) -> bool {
                    ! self.is_high()
                }
            }
        )+
    }
}

gpio![
    P0_0: (p0_0, 0, 0, Input, dirp0, clrp00, setp0),
    P0_1: (p0_1, 0, 1, Input, dirp1, clrp01, setp1),
    P0_2: (p0_2, 0, 2, Input, dirp2, clrp02, setp2),
    P0_3: (p0_3, 0, 3, Input, dirp3, clrp03, setp3),
    P0_4: (p0_4, 0, 4, Input, dirp4, clrp04, setp4),
    P0_5: (p0_5, 0, 5, Input, dirp5, clrp05, setp5),
    P0_6: (p0_6, 0, 6, Input, dirp6, clrp06, setp6),
    P0_7: (p0_7, 0, 7, Input, dirp7, clrp07, setp7),
    P0_8: (p0_8, 0, 8, Input, dirp8, clrp08, setp8),
    P0_9: (p0_9, 0, 9, Input, dirp9, clrp09, setp9),
    P0_10: (p0_10, 0, 10, Input, dirp10, clrp010, setp10),
    P0_11: (p0_11, 0, 11, Input, dirp11, clrp011, setp11),
    P0_12: (p0_12, 0, 12, Input, dirp12, clrp012, setp12),
    P0_13: (p0_13, 0, 13, Input, dirp13, clrp013, setp13),
    P0_14: (p0_14, 0, 14, Input, dirp14, clrp014, setp14),
    P0_15: (p0_15, 0, 15, Input, dirp15, clrp015, setp15),
    P0_16: (p0_16, 0, 16, Input, dirp16, clrp016, setp16),
    P0_17: (p0_17, 0, 17, Input, dirp17, clrp017, setp17),
    P0_18: (p0_18, 0, 18, Input, dirp18, clrp018, setp18),
    P0_19: (p0_19, 0, 19, Input, dirp19, clrp019, setp19),
    P0_20: (p0_20, 0, 20, Input, dirp20, clrp020, setp20),
    P0_21: (p0_21, 0, 21, Input, dirp21, clrp021, setp21),
    P0_22: (p0_22, 0, 22, Input, dirp22, clrp022, setp22),
    P0_23: (p0_23, 0, 23, Input, dirp23, clrp023, setp23),
    P0_24: (p0_24, 0, 24, Input, dirp24, clrp024, setp24),
    P0_25: (p0_25, 0, 25, Input, dirp25, clrp025, setp25),
    P0_26: (p0_26, 0, 26, Input, dirp26, clrp026, setp26),
    P0_27: (p0_27, 0, 27, Input, dirp27, clrp027, setp27),
    P0_28: (p0_28, 0, 28, Input, dirp28, clrp028, setp28),
    P0_29: (p0_29, 0, 29, Input, dirp29, clrp029, setp29),
    P0_30: (p0_30, 0, 30, Input, dirp30, clrp030, setp30),
    P0_31: (p0_31, 0, 31, Input, dirp31, clrp031, setp31),
    P1_0: (p1_0, 1, 0, Input, dirp0, clrp00, setp0),
    P1_1: (p1_1, 1, 1, Input, dirp1, clrp01, setp1),
    P1_2: (p1_2, 1, 2, Input, dirp2, clrp02, setp2),
    P1_3: (p1_3, 1, 3, Input, dirp3, clrp03, setp3),
    P1_4: (p1_4, 1, 4, Input, dirp4, clrp04, setp4),
    P1_5: (p1_5, 1, 5, Input, dirp5, clrp05, setp5),
    P1_6: (p1_6, 1, 6, Input, dirp6, clrp06, setp6),
    P1_7: (p1_7, 1, 7, Input, dirp7, clrp07, setp7),
    P1_8: (p1_8, 1, 8, Input, dirp8, clrp08, setp8),
    P1_9: (p1_9, 1, 9, Input, dirp9, clrp09, setp9),
    P1_10: (p1_10, 1, 10, Input, dirp10, clrp010, setp10),
    P1_11: (p1_11, 1, 11, Input, dirp11, clrp011, setp11),
    P1_12: (p1_12, 1, 12, Input, dirp12, clrp012, setp12),
    P1_13: (p1_13, 1, 13, Input, dirp13, clrp013, setp13),
    P1_14: (p1_14, 1, 14, Input, dirp14, clrp014, setp14),
    P1_15: (p1_15, 1, 15, Input, dirp15, clrp015, setp15),
    P1_16: (p1_16, 1, 16, Input, dirp16, clrp016, setp16),
    P1_17: (p1_17, 1, 17, Input, dirp17, clrp017, setp17),
    P1_18: (p1_18, 1, 18, Input, dirp18, clrp018, setp18),
    P1_19: (p1_19, 1, 19, Input, dirp19, clrp019, setp19),
    P1_20: (p1_20, 1, 20, Input, dirp20, clrp020, setp20),
    P1_21: (p1_21, 1, 21, Input, dirp21, clrp021, setp21),
    P1_22: (p1_22, 1, 22, Input, dirp22, clrp022, setp22),
    P1_23: (p1_23, 1, 23, Input, dirp23, clrp023, setp23),
    P1_24: (p1_24, 1, 24, Input, dirp24, clrp024, setp24),
    P1_25: (p1_25, 1, 25, Input, dirp25, clrp025, setp25),
    P1_26: (p1_26, 1, 26, Input, dirp26, clrp026, setp26),
    P1_27: (p1_27, 1, 27, Input, dirp27, clrp027, setp27),
    P1_28: (p1_28, 1, 28, Input, dirp28, clrp028, setp28),
    P1_29: (p1_29, 1, 29, Input, dirp29, clrp029, setp29),
    P1_30: (p1_30, 1, 30, Input, dirp30, clrp030, setp30),
    P1_31: (p1_31, 1, 31, Input, dirp31, clrp031, setp31),
    P2_0: (p2_0, 2, 0, Input, dirp0, clrp00, setp0),
    P2_1: (p2_1, 2, 1, Input, dirp1, clrp01, setp1),
    P2_2: (p2_2, 2, 2, Input, dirp2, clrp02, setp2),
    P2_3: (p2_3, 2, 3, Input, dirp3, clrp03, setp3),
    P2_4: (p2_4, 2, 4, Input, dirp4, clrp04, setp4),
    P2_5: (p2_5, 2, 5, Input, dirp5, clrp05, setp5),
    P2_6: (p2_6, 2, 6, Input, dirp6, clrp06, setp6),
    P2_7: (p2_7, 2, 7, Input, dirp7, clrp07, setp7),
    P2_8: (p2_8, 2, 8, Input, dirp8, clrp08, setp8),
    P2_9: (p2_9, 2, 9, Input, dirp9, clrp09, setp9),
    P2_10: (p2_10, 2, 10, Input, dirp10, clrp010, setp10),
    P2_11: (p2_11, 2, 11, Input, dirp11, clrp011, setp11),
    P2_12: (p2_12, 2, 12, Input, dirp12, clrp012, setp12),
    P2_13: (p2_13, 2, 13, Input, dirp13, clrp013, setp13),
    P2_14: (p2_14, 2, 14, Input, dirp14, clrp014, setp14),
    P2_15: (p2_15, 2, 15, Input, dirp15, clrp015, setp15),
    P2_16: (p2_16, 2, 16, Input, dirp16, clrp016, setp16),
    P2_17: (p2_17, 2, 17, Input, dirp17, clrp017, setp17),
    P2_18: (p2_18, 2, 18, Input, dirp18, clrp018, setp18),
    P2_19: (p2_19, 2, 19, Input, dirp19, clrp019, setp19),
    P2_20: (p2_20, 2, 20, Input, dirp20, clrp020, setp20),
    P2_21: (p2_21, 2, 21, Input, dirp21, clrp021, setp21),
    P2_22: (p2_22, 2, 22, Input, dirp22, clrp022, setp22),
    P2_23: (p2_23, 2, 23, Input, dirp23, clrp023, setp23),
    P2_24: (p2_24, 2, 24, Input, dirp24, clrp024, setp24),
    P2_25: (p2_25, 2, 25, Input, dirp25, clrp025, setp25),
    P2_26: (p2_26, 2, 26, Input, dirp26, clrp026, setp26),
    P2_27: (p2_27, 2, 27, Input, dirp27, clrp027, setp27),
    P2_28: (p2_28, 2, 28, Input, dirp28, clrp028, setp28),
    P2_29: (p2_29, 2, 29, Input, dirp29, clrp029, setp29),
    P2_30: (p2_30, 2, 30, Input, dirp30, clrp030, setp30),
    P2_31: (p2_31, 2, 31, Input, dirp31, clrp031, setp31),
    P3_0: (p3_0, 3, 0, Input, dirp0, clrp00, setp0),
    P3_1: (p3_1, 3, 1, Input, dirp1, clrp01, setp1),
    P3_2: (p3_2, 3, 2, Input, dirp2, clrp02, setp2),
    P3_3: (p3_3, 3, 3, Input, dirp3, clrp03, setp3),
    P3_4: (p3_4, 3, 4, Input, dirp4, clrp04, setp4),
    P3_5: (p3_5, 3, 5, Input, dirp5, clrp05, setp5),
    P3_6: (p3_6, 3, 6, Input, dirp6, clrp06, setp6),
    P3_7: (p3_7, 3, 7, Input, dirp7, clrp07, setp7),
    P3_8: (p3_8, 3, 8, Input, dirp8, clrp08, setp8),
    P3_9: (p3_9, 3, 9, Input, dirp9, clrp09, setp9),
    P3_10: (p3_10, 3, 10, Input, dirp10, clrp010, setp10),
    P3_11: (p3_11, 3, 11, Input, dirp11, clrp011, setp11),
    P3_12: (p3_12, 3, 12, Input, dirp12, clrp012, setp12),
    P3_13: (p3_13, 3, 13, Input, dirp13, clrp013, setp13),
    P3_14: (p3_14, 3, 14, Input, dirp14, clrp014, setp14),
    P3_15: (p3_15, 3, 15, Input, dirp15, clrp015, setp15),
    P3_16: (p3_16, 3, 16, Input, dirp16, clrp016, setp16),
    P3_17: (p3_17, 3, 17, Input, dirp17, clrp017, setp17),
    P3_18: (p3_18, 3, 18, Input, dirp18, clrp018, setp18),
    P3_19: (p3_19, 3, 19, Input, dirp19, clrp019, setp19),
    P3_20: (p3_20, 3, 20, Input, dirp20, clrp020, setp20),
    P3_21: (p3_21, 3, 21, Input, dirp21, clrp021, setp21),
    P3_22: (p3_22, 3, 22, Input, dirp22, clrp022, setp22),
    P3_23: (p3_23, 3, 23, Input, dirp23, clrp023, setp23),
    P3_24: (p3_24, 3, 24, Input, dirp24, clrp024, setp24),
    P3_25: (p3_25, 3, 25, Input, dirp25, clrp025, setp25),
    P3_26: (p3_26, 3, 26, Input, dirp26, clrp026, setp26),
    P3_27: (p3_27, 3, 27, Input, dirp27, clrp027, setp27),
    P3_28: (p3_28, 3, 28, Input, dirp28, clrp028, setp28),
    P3_29: (p3_29, 3, 29, Input, dirp29, clrp029, setp29),
    P3_30: (p3_30, 3, 30, Input, dirp30, clrp030, setp30),
    P3_31: (p3_31, 3, 31, Input, dirp31, clrp031, setp31),
    P4_0: (p4_0, 4, 0, Input, dirp0, clrp00, setp0),
    P4_1: (p4_1, 4, 1, Input, dirp1, clrp01, setp1),
    P4_2: (p4_2, 4, 2, Input, dirp2, clrp02, setp2),
    P4_3: (p4_3, 4, 3, Input, dirp3, clrp03, setp3),
    P4_4: (p4_4, 4, 4, Input, dirp4, clrp04, setp4),
    P4_5: (p4_5, 4, 5, Input, dirp5, clrp05, setp5),
    P4_6: (p4_6, 4, 6, Input, dirp6, clrp06, setp6),
    P4_7: (p4_7, 4, 7, Input, dirp7, clrp07, setp7),
    P4_8: (p4_8, 4, 8, Input, dirp8, clrp08, setp8),
    P4_9: (p4_9, 4, 9, Input, dirp9, clrp09, setp9),
    P4_10: (p4_10, 4, 10, Input, dirp10, clrp010, setp10),
    P4_11: (p4_11, 4, 11, Input, dirp11, clrp011, setp11),
    P4_12: (p4_12, 4, 12, Input, dirp12, clrp012, setp12),
    P4_13: (p4_13, 4, 13, Input, dirp13, clrp013, setp13),
    P4_14: (p4_14, 4, 14, Input, dirp14, clrp014, setp14),
    P4_15: (p4_15, 4, 15, Input, dirp15, clrp015, setp15),
    P4_16: (p4_16, 4, 16, Input, dirp16, clrp016, setp16),
    P4_17: (p4_17, 4, 17, Input, dirp17, clrp017, setp17),
    P4_18: (p4_18, 4, 18, Input, dirp18, clrp018, setp18),
    P4_19: (p4_19, 4, 19, Input, dirp19, clrp019, setp19),
    P4_20: (p4_20, 4, 20, Input, dirp20, clrp020, setp20),
    P4_21: (p4_21, 4, 21, Input, dirp21, clrp021, setp21),
    P4_22: (p4_22, 4, 22, Input, dirp22, clrp022, setp22),
    P4_23: (p4_23, 4, 23, Input, dirp23, clrp023, setp23),
    P4_24: (p4_24, 4, 24, Input, dirp24, clrp024, setp24),
    P4_25: (p4_25, 4, 25, Input, dirp25, clrp025, setp25),
    P4_26: (p4_26, 4, 26, Input, dirp26, clrp026, setp26),
    P4_27: (p4_27, 4, 27, Input, dirp27, clrp027, setp27),
    P4_28: (p4_28, 4, 28, Input, dirp28, clrp028, setp28),
    P4_29: (p4_29, 4, 29, Input, dirp29, clrp029, setp29),
    P4_30: (p4_30, 4, 30, Input, dirp30, clrp030, setp30),
    P4_31: (p4_31, 4, 31, Input, dirp31, clrp031, setp31),
    P5_0: (p5_0, 5, 0, Input, dirp0, clrp00, setp0),
    P5_1: (p5_1, 5, 1, Input, dirp1, clrp01, setp1),
    P5_2: (p5_2, 5, 2, Input, dirp2, clrp02, setp2),
    P5_3: (p5_3, 5, 3, Input, dirp3, clrp03, setp3),
    P5_4: (p5_4, 5, 4, Input, dirp4, clrp04, setp4),
    P5_5: (p5_5, 5, 5, Input, dirp5, clrp05, setp5),
    P5_6: (p5_6, 5, 6, Input, dirp6, clrp06, setp6),
    P5_7: (p5_7, 5, 7, Input, dirp7, clrp07, setp7),
    P5_8: (p5_8, 5, 8, Input, dirp8, clrp08, setp8),
    P5_9: (p5_9, 5, 9, Input, dirp9, clrp09, setp9),
    P5_10: (p5_10, 5, 10, Input, dirp10, clrp010, setp10),
    P5_11: (p5_11, 5, 11, Input, dirp11, clrp011, setp11),
    P5_12: (p5_12, 5, 12, Input, dirp12, clrp012, setp12),
    P5_13: (p5_13, 5, 13, Input, dirp13, clrp013, setp13),
    P5_14: (p5_14, 5, 14, Input, dirp14, clrp014, setp14),
    P5_15: (p5_15, 5, 15, Input, dirp15, clrp015, setp15),
    P5_16: (p5_16, 5, 16, Input, dirp16, clrp016, setp16),
    P5_17: (p5_17, 5, 17, Input, dirp17, clrp017, setp17),
    P5_18: (p5_18, 5, 18, Input, dirp18, clrp018, setp18),
    P5_19: (p5_19, 5, 19, Input, dirp19, clrp019, setp19),
    P5_20: (p5_20, 5, 20, Input, dirp20, clrp020, setp20),
    P5_21: (p5_21, 5, 21, Input, dirp21, clrp021, setp21),
    P5_22: (p5_22, 5, 22, Input, dirp22, clrp022, setp22),
    P5_23: (p5_23, 5, 23, Input, dirp23, clrp023, setp23),
    P5_24: (p5_24, 5, 24, Input, dirp24, clrp024, setp24),
    P5_25: (p5_25, 5, 25, Input, dirp25, clrp025, setp25),
    P5_26: (p5_26, 5, 26, Input, dirp26, clrp026, setp26),
    P5_27: (p5_27, 5, 27, Input, dirp27, clrp027, setp27),
    P5_28: (p5_28, 5, 28, Input, dirp28, clrp028, setp28),
    P5_29: (p5_29, 5, 29, Input, dirp29, clrp029, setp29),
    P5_30: (p5_30, 5, 30, Input, dirp30, clrp030, setp30),
    P5_31: (p5_31, 5, 31, Input, dirp31, clrp031, setp31),
    P6_0: (p6_0, 6, 0, Input, dirp0, clrp00, setp0),
    P6_1: (p6_1, 6, 1, Input, dirp1, clrp01, setp1),
    P6_2: (p6_2, 6, 2, Input, dirp2, clrp02, setp2),
    P6_3: (p6_3, 6, 3, Input, dirp3, clrp03, setp3),
    P6_4: (p6_4, 6, 4, Input, dirp4, clrp04, setp4),
    P6_5: (p6_5, 6, 5, Input, dirp5, clrp05, setp5),
    P6_6: (p6_6, 6, 6, Input, dirp6, clrp06, setp6),
    P6_7: (p6_7, 6, 7, Input, dirp7, clrp07, setp7),
    P6_8: (p6_8, 6, 8, Input, dirp8, clrp08, setp8),
    P6_9: (p6_9, 6, 9, Input, dirp9, clrp09, setp9),
    P6_10: (p6_10, 6, 10, Input, dirp10, clrp010, setp10),
    P6_11: (p6_11, 6, 11, Input, dirp11, clrp011, setp11),
    P6_12: (p6_12, 6, 12, Input, dirp12, clrp012, setp12),
    P6_13: (p6_13, 6, 13, Input, dirp13, clrp013, setp13),
    P6_14: (p6_14, 6, 14, Input, dirp14, clrp014, setp14),
    P6_15: (p6_15, 6, 15, Input, dirp15, clrp015, setp15),
    P6_16: (p6_16, 6, 16, Input, dirp16, clrp016, setp16),
    P6_17: (p6_17, 6, 17, Input, dirp17, clrp017, setp17),
    P6_18: (p6_18, 6, 18, Input, dirp18, clrp018, setp18),
    P6_19: (p6_19, 6, 19, Input, dirp19, clrp019, setp19),
    P6_20: (p6_20, 6, 20, Input, dirp20, clrp020, setp20),
    P6_21: (p6_21, 6, 21, Input, dirp21, clrp021, setp21),
    P6_22: (p6_22, 6, 22, Input, dirp22, clrp022, setp22),
    P6_23: (p6_23, 6, 23, Input, dirp23, clrp023, setp23),
    P6_24: (p6_24, 6, 24, Input, dirp24, clrp024, setp24),
    P6_25: (p6_25, 6, 25, Input, dirp25, clrp025, setp25),
    P6_26: (p6_26, 6, 26, Input, dirp26, clrp026, setp26),
    P6_27: (p6_27, 6, 27, Input, dirp27, clrp027, setp27),
    P6_28: (p6_28, 6, 28, Input, dirp28, clrp028, setp28),
    P6_29: (p6_29, 6, 29, Input, dirp29, clrp029, setp29),
    P6_30: (p6_30, 6, 30, Input, dirp30, clrp030, setp30),
    P6_31: (p6_31, 6, 31, Input, dirp31, clrp031, setp31),
    P7_0: (p7_0, 7, 0, Input, dirp0, clrp00, setp0),
    P7_1: (p7_1, 7, 1, Input, dirp1, clrp01, setp1),
    P7_2: (p7_2, 7, 2, Input, dirp2, clrp02, setp2),
    P7_3: (p7_3, 7, 3, Input, dirp3, clrp03, setp3),
    P7_4: (p7_4, 7, 4, Input, dirp4, clrp04, setp4),
    P7_5: (p7_5, 7, 5, Input, dirp5, clrp05, setp5),
    P7_6: (p7_6, 7, 6, Input, dirp6, clrp06, setp6),
    P7_7: (p7_7, 7, 7, Input, dirp7, clrp07, setp7),
    P7_8: (p7_8, 7, 8, Input, dirp8, clrp08, setp8),
    P7_9: (p7_9, 7, 9, Input, dirp9, clrp09, setp9),
    P7_10: (p7_10, 7, 10, Input, dirp10, clrp010, setp10),
    P7_11: (p7_11, 7, 11, Input, dirp11, clrp011, setp11),
    P7_12: (p7_12, 7, 12, Input, dirp12, clrp012, setp12),
    P7_13: (p7_13, 7, 13, Input, dirp13, clrp013, setp13),
    P7_14: (p7_14, 7, 14, Input, dirp14, clrp014, setp14),
    P7_15: (p7_15, 7, 15, Input, dirp15, clrp015, setp15),
    P7_16: (p7_16, 7, 16, Input, dirp16, clrp016, setp16),
    P7_17: (p7_17, 7, 17, Input, dirp17, clrp017, setp17),
    P7_18: (p7_18, 7, 18, Input, dirp18, clrp018, setp18),
    P7_19: (p7_19, 7, 19, Input, dirp19, clrp019, setp19),
    P7_20: (p7_20, 7, 20, Input, dirp20, clrp020, setp20),
    P7_21: (p7_21, 7, 21, Input, dirp21, clrp021, setp21),
    P7_22: (p7_22, 7, 22, Input, dirp22, clrp022, setp22),
    P7_23: (p7_23, 7, 23, Input, dirp23, clrp023, setp23),
    P7_24: (p7_24, 7, 24, Input, dirp24, clrp024, setp24),
    P7_25: (p7_25, 7, 25, Input, dirp25, clrp025, setp25),
    P7_26: (p7_26, 7, 26, Input, dirp26, clrp026, setp26),
    P7_27: (p7_27, 7, 27, Input, dirp27, clrp027, setp27),
    P7_28: (p7_28, 7, 28, Input, dirp28, clrp028, setp28),
    P7_29: (p7_29, 7, 29, Input, dirp29, clrp029, setp29),
    P7_30: (p7_30, 7, 30, Input, dirp30, clrp030, setp30),
    P7_31: (p7_31, 7, 31, Input, dirp31, clrp031, setp31),
    P8_0: (p8_0, 8, 0, Input, dirp0, clrp00, setp0),
    P8_1: (p8_1, 8, 1, Input, dirp1, clrp01, setp1),
    P8_2: (p8_2, 8, 2, Input, dirp2, clrp02, setp2),
    P8_3: (p8_3, 8, 3, Input, dirp3, clrp03, setp3),
    P8_4: (p8_4, 8, 4, Input, dirp4, clrp04, setp4),
    P8_5: (p8_5, 8, 5, Input, dirp5, clrp05, setp5),
    P8_6: (p8_6, 8, 6, Input, dirp6, clrp06, setp6),
    P8_7: (p8_7, 8, 7, Input, dirp7, clrp07, setp7),
    P8_8: (p8_8, 8, 8, Input, dirp8, clrp08, setp8),
    P8_9: (p8_9, 8, 9, Input, dirp9, clrp09, setp9),
    P8_10: (p8_10, 8, 10, Input, dirp10, clrp010, setp10),
    P8_11: (p8_11, 8, 11, Input, dirp11, clrp011, setp11),
    P8_12: (p8_12, 8, 12, Input, dirp12, clrp012, setp12),
    P8_13: (p8_13, 8, 13, Input, dirp13, clrp013, setp13),
    P8_14: (p8_14, 8, 14, Input, dirp14, clrp014, setp14),
    P8_15: (p8_15, 8, 15, Input, dirp15, clrp015, setp15),
    P8_16: (p8_16, 8, 16, Input, dirp16, clrp016, setp16),
    P8_17: (p8_17, 8, 17, Input, dirp17, clrp017, setp17),
    P8_18: (p8_18, 8, 18, Input, dirp18, clrp018, setp18),
    P8_19: (p8_19, 8, 19, Input, dirp19, clrp019, setp19),
    P8_20: (p8_20, 8, 20, Input, dirp20, clrp020, setp20),
    P8_21: (p8_21, 8, 21, Input, dirp21, clrp021, setp21),
    P8_22: (p8_22, 8, 22, Input, dirp22, clrp022, setp22),
    P8_23: (p8_23, 8, 23, Input, dirp23, clrp023, setp23),
    P8_24: (p8_24, 8, 24, Input, dirp24, clrp024, setp24),
    P8_25: (p8_25, 8, 25, Input, dirp25, clrp025, setp25),
    P8_26: (p8_26, 8, 26, Input, dirp26, clrp026, setp26),
    P8_27: (p8_27, 8, 27, Input, dirp27, clrp027, setp27),
    P8_28: (p8_28, 8, 28, Input, dirp28, clrp028, setp28),
    P8_29: (p8_29, 8, 29, Input, dirp29, clrp029, setp29),
    P8_30: (p8_30, 8, 30, Input, dirp30, clrp030, setp30),
    P8_31: (p8_31, 8, 31, Input, dirp31, clrp031, setp31),
];
