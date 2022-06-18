// # DS1820 sensor driver
#![cfg_attr(not(test), no_std)]

mod onewire;
pub use onewire::{Delay, Ds1820Error, InputOutputPin};

pub trait Ds1820Reading: internal::FromRaw + Sized {
    fn read<P, E>(delay: &mut dyn Delay, pin: &mut P) -> Result<Self, onewire::Ds1820Error<E>>
    where
        P: InputOutputPin<E>,
    {
        onewire::read_raw(delay, pin).map(Self::raw_to_reading)
    }
}

mod internal {
    pub trait FromRaw {
        fn raw_to_reading(bytes: [u8; 4]) -> Self;
    }
}

pub mod ds1820 {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Reading {
        pub temperature: i8,
        pub relative_humidity: u8,
    }

    impl internal::FromRaw for Reading {
        fn raw_to_reading(bytes: [u8; 4]) -> Reading {
            let [rh, _, temp_signed, _] = bytes;
            let temp = {
                let (signed, magnitude) = convert_signed(temp_signed);
                let temp_sign = if signed { -1 } else { 1 };
                temp_sign * magnitude as i8
            };
            Reading {
                temperature: temp,
                relative_humidity: rh,
            }
        }
    }

    impl Ds1820Reading for Reading {}

    #[test]
    fn test_raw_to_reading() {
        use super::internal::FromRaw;

        assert_eq!(
            Reading::raw_to_reading([0x32, 0, 0x1B, 0]),
            Reading {
                temperature: 27,
                relative_humidity: 50
            }
        );
        assert_eq!(
            Reading::raw_to_reading([0x80, 0, 0x83, 0]),
            Reading {
                temperature: -3,
                relative_humidity: 128
            }
        );
    }
}

fn convert_signed(signed: u8) -> (bool, u8) {
    let sign = signed & 0x80 != 0;
    let magnitude = signed & 0x7F;
    (sign, magnitude)
}

#[test]
fn test_convert_signed() {
    assert_eq!(convert_signed(0x13), (false, 0x13));
    assert_eq!(convert_signed(0x93), (true, 0x13));
}
