enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait Timer {
    fn time(&self) -> u32;
}

impl Timer for TrafficLight {
    fn time(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    println!("Red light lasts for {} seconds.", red.time());
    println!("Yellow light lasts for {} seconds.", yellow.time());
    println!("Green light lasts for {} seconds.", green.time());
}
