use crate::gpio::OutputPinRole;
use std::time::Duration;
use std::thread::sleep;

pub fn buzz() -> &'static str {
    let pin = OutputPinRole::Buzzer::get_pin();
    let interval = 10;
    let reps = 100;
    for i in 0..reps {
        pin.set_high();
        sleep(Duration::from_millis(interval));
        pin.set_low();
        sleep(Duration::from_millis(interval));
    }
    "Buzzing request sent"
}

