// import std::fmt::Display

enum TrafficLight {
    RED, YELLOW, GREEN,
}

impl TrafficLight {
    fn time(self) -> u8 {
        match self {
            RED => {
                println!("RED");
                30
            }
            YELLOW => {
                println!("YELLOW");
                3
            }
            GREEN => {
                println!("GREEN");
                60
            }
        }
    }
}

fn main() {
    let red:TrafficLight = TrafficLight::RED;
    let green:TrafficLight = TrafficLight::GREEN;
    let yellow:TrafficLight = TrafficLight::YELLOW;

    println!("{}", red.time());
    println!("{}", green.time());
    println!("{}", yellow.time());
}
