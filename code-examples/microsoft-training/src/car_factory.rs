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
pub struct Car<'color> {
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

pub fn build<'color>(color: &'color String, gears: i8, miles: u32) -> Car<'color> {
    let color_hex: String;

    if let Some(color_for_sure) = COLORS.get(color) {
        color_hex = color_for_sure.to_owned();
    } else {
        color_hex = String::from("#FFFAFA");
    }

    let mut motor = Transmission::Manual;
    let mut roof = false;

    if gears % 3 == 0 {
        println!("GOES IN HERE");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_car() {
        let color = String::from("IndianRed");
        let gears = 6;
        let miles = 100;
        let expected = Car {
            color_name: &color,
            color_hex: String::from("#CD5C5C"),
            motor: Transmission::SemiAutomatic,
            roof: true,
            mileage: (Age::Used, miles),
        };

        let actual = build(&color, gears, miles);
        assert_eq!(expected, actual);
    }

    #[test]
    fn build_car_with_unknown_color() {
        let color = String::from("UnknownColor");
        let gears = 9;
        let miles = 100;
        let expected = Car {
            color_name: &color,
            color_hex: String::from("#FFFAFA"),
            motor: Transmission::Automatic,
            roof: true,
            mileage: (Age::Used, miles),
        };

        let actual = build(&color, gears, miles);
        assert_eq!(expected, actual);
    }

    #[test]
    fn run_quality_car() {
        let quality = car_quality(0);
        assert_eq!(quality, (Age::New, 0));
    }

    #[test]
    #[should_panic]
    fn fail_quality_car() {
        let quality = car_quality(0);
        assert_eq!(quality, (Age::Used, 0));
    }
}
