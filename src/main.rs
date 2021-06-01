enum TrafficLights{
	Red,
	Yellow,
	Green,
}
trait Signal{
	fn time(&self) -> u8;
}
impl Signal for TrafficLights{
	fn time(&self) -> u8{
		match self {
			TrafficLights::Red => 30,
			TrafficLights::Yellow => 3,
			TrafficLights::Green => 60,
		}
	}
}

fn main() {
	let r = TrafficLights::Red;
    println!("Red light is: {} second",r.time());
	let y = TrafficLights::Yellow;
	println!("Yellow light is: {} second",y.time());
	let g = TrafficLights::Green;
	println!("Green light is: {} second",g.time());
}
