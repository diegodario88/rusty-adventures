use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref COLORS: HashMap<String, String> = {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(String::from("IndianRed"), String::from("#CD5C5C"));
        map.insert(String::from("Salmon"), String::from("#FA8072"));
        map.insert(String::from("RosyBrown"), String::from("#BC8F8F"));

        return map;
    };
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    Automatic,
    SemiAutomatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    mileage: (Age, u32),
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles > 0 {
        return (Age::Used, miles);
    }

    (Age::New, miles)
}

fn car_factory(color: String, gears: i8, miles: u32) -> Car {
    let hex_color: String;

    if let Some(color_for_sure) = COLORS.get(&color) {
        hex_color = color_for_sure.to_owned();
    } else {
        hex_color = String::from("#FFFAFA");
    }

    let mut motor = Transmission::Manual;
    let mut roof = false;

    if gears % 3 == 0 {
        motor = Transmission::Automatic;
        roof = true;
    }

    if gears % 2 == 0 {
        motor = Transmission::SemiAutomatic
    }

    Car {
        color: hex_color,
        motor,
        roof,
        mileage: car_quality(miles),
    }
}

fn main() {
    for _ in 1..10 {
        println!(
            "Car made: {:?}",
            car_factory(String::from("IndianRed"), 12, 35)
        )
    }

    let car1 = car_factory(String::from("Salmon"), 5, 3500);
    println!("Car made: {:?}", car1);

    let car2 = car_factory(String::from("RosyBrown"), -128, 0);
    println!("Car made: {:?}", car2);
}
