use crate::gpio::OutputPinRole;
use std::thread::sleep;
use std::time::Duration;

pub async fn buzz() -> &'static str {
    let mut pin = OutputPinRole::get_pin(&OutputPinRole::Buzzer);
    let interval = 10;
    let reps = 100;
    for _ in 0..reps {
        pin.set_high();
        sleep(Duration::from_millis(interval));
        pin.set_low();
        sleep(Duration::from_millis(interval));
    }
    "Buzzing request sent"
}
