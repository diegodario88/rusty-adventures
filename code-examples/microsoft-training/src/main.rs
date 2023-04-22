mod car_factory;
mod counter;

fn main() {
    let counter = counter::CarDemand::new(10);

    let indian_red_color = String::from("IndianRed");
    let rosy_brown = String::from("RosyBrown");
    let salmon = String::from("Salmon");

    for _ in counter {
        println!(
            "Car made: {:?}",
            car_factory::build(&indian_red_color, 9, 35)
        )
    }

    let golf = car_factory::build(&salmon, -128, 3500);
    println!("Car made: {:?}", golf);

    let corola = car_factory::build(&rosy_brown, 5, 0);
    println!("Car made: {:?}", corola);
}
