use crate::gpio::OutputPinRole;

pub async fn fan_on() -> &'static str {
    let mut pin = OutputPinRole::get_pin(&OutputPinRole::Fan);
    println!("Turning fan on...");
    pin.set_high();
    
    "Fan has been requested to turn on"
}

pub async fn fan_off() -> &'static str {
    let mut pin = OutputPinRole::get_pin(&OutputPinRole::Fan);
    println!("Turning fan off...");
    pin.set_low();

    "Fan has been requested to turn off"
}