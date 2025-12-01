pub mod buzzer;
use rppal::gpio::{OutputPin, Gpio};

#[derive(Debug, Clone, Copy)]
pub enum OutputPinRole {
    Buzzer
}

impl OutputPinRole {
    pub fn get_type(&self) -> PinType {
        match self {
            OutputPinRole::Buzzer => Output
        }
    }

    pub fn get_bcm(&self) -> i32 {
        match self {
            OutputPinRole::Buzzer => 17
        }
    }

    pub fn get_pin(&self) -> OutputPin {
        Gpio::new()?.get(OutputPin::Led.bcm())?.into_output()
    }
}


