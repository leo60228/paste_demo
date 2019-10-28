#[derive(Debug, PartialEq, Eq)]
pub enum GpioPort {
    A,
    B,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GpioPin {
    P1,
    P2,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Gpio {
    pub port: GpioPort,
    pub pin: GpioPin,
}

#[macro_export]
macro_rules! gpio {
    ($vis:vis $port:ident $pin:tt) => {
        ::paste::item! {
            $vis static [<GPIO_ $port $pin>]: Gpio = Gpio {
                port: GpioPort::$port,
                pin: ::paste::expr!(GpioPin::[<P $pin>]),
            };
        }
    }
}

gpio!(pub A 1);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn assert_gpio() {
        assert_eq!(GPIO_A1, Gpio {
            port: GpioPort::A,
            pin: GpioPin::P1,
        });
    }
}
