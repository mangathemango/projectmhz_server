pub mod buzzer;
pub mod fan;
use rppal::gpio::{OutputPin, Gpio};

#[derive(Debug, Clone, Copy)]
pub enum OutputPinRole {
    Buzzer,
    Fan
}

impl OutputPinRole  {
    pub fn get_bcm(&self) -> u8 {
        match self {
            OutputPinRole::Buzzer => 17,
            OutputPinRole::Fan => 27
        }
    }

    pub fn get_pin(&self) -> OutputPin {
        Gpio::new()
            .expect("Unable to get GPIO")
            .get(self.get_bcm())
            .expect(format!("Cannot get pin {}",self.get_bcm()).as_str())
            .into_output()
    }
}


