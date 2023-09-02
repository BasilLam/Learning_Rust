use std::io;

fn cel_to_far(t: f32) -> f32 {
    return t * 9.0 / 5.0 + 32.0;
}

fn far_to_cel(t: f32) -> f32 {
    return (t - 32.0) * 5.0 / 9.0;
}


fn main() {

    println! ("Please enter temperature.");
    let t : f32;
    loop {
        let mut temperature_input = String::new();
        io::stdin()
                .read_line(& mut temperature_input)
                .expect("Failed to load temperature");
        if let Ok(val) = temperature_input.trim().parse::<f32>() {
            t = val;
            println! ("Temperature input is {}", t);
            break;
        } else {
            println!("Please input a valid number")
        }
    }
    
    println! ("Please enter unit of measurement Cc/Ff");
    let mut character : char;
    loop {
        let mut unit_input = String::new();
        io::stdin()
                    .read_line(&mut unit_input)
                    .expect("Failed to read line");
        character = unit_input.trim()
                                    .chars()
                                    .next()
                                    .unwrap();
        if "CcFf".contains(character) {
            println!("You entered a valid unit of measurement: {}", character);
            break;
        } else {
            println!("Invalid character entered! Please enter valid unit of measurement (Cc/Ff)");
        }
    }
    
    if character == 'C' || character == 'c' {
        let temp: f32 = cel_to_far(t);
        println!("Est. Fahreneit is {}, In Celsius Temperature: {}", temp, t);
    }

    if character == 'F' || character == 'f' {
        let temp: f32 = far_to_cel(t);
        println!("Est. Celsius is {}, In Fahreneit Temperature: {}", temp, t);
    }




    
    //println! ("Please enter temperature unit. Cc/Ff")
    
    //let mut temperature_unit_input = String::new();

}