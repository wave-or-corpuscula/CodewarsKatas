fn validate_pin(pin: &str) -> bool {
    if pin.len() != 4 && pin.len() != 6 {
        return false;
    }
    for ch in pin.chars() {
        if ((ch as u8) < 48) || ((ch as u8) > 57) {
            return false;
        }
    }
    return true;
}


fn validate_pin_cooler(pin: &str) -> bool {
    pin.chars().all(|c| c.is_digit(10)) && (pin.len() == 4 || pin.len() == 6)
}

fn main() {
    println!("result: {}", validate_pin_cooler("1234"));
}
