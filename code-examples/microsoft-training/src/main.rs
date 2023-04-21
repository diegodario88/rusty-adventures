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
struct Car<'color> {
    color_name: &'color String,
    color_hex: String,
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

fn car_factory<'color>(color: &'color String, gears: i8, miles: u32) -> Car<'color> {
    let color_hex: String;

    if let Some(color_for_sure) = COLORS.get(color) {
        color_hex = color_for_sure.to_owned();
    } else {
        color_hex = String::from("#FFFAFA");
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
        color_name: color,
        color_hex,
        motor,
        roof,
        mileage: car_quality(miles),
    }
}

fn main() {
    let indian_red_color = String::from("IndianRed");
    let rosy_brown = String::from("RosyBrown");
    let salmon = String::from("Salmon");

    for _ in 1..10 {
        println!("Car made: {:?}", car_factory(&indian_red_color, 12, 35))
    }

    let golf = car_factory(&salmon, -128, 3500);
    println!("Car made: {:?}", golf);

    let corola = car_factory(&rosy_brown, 5, 0);
    println!("Car made: {:?}", corola);
}
