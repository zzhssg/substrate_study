#[derive(Debug)]
enum TrafficLight {
    RED(String),
    GREEN(String),
    Yellow(String),
}

pub trait Time{
    fn print_time( &self) -> String ;
}

impl Time for TrafficLight {
    fn print_time(&self) -> String {
        match self {
            TrafficLight::Some(time) => {
                println!("{:?}",time)
            }
       
        }
        let x= String::from("123");
        x

    }
}
fn main() {
    let x = TrafficLight::GREEN(String::from("60s"));
    let y = TrafficLight::GREEN(String::from("10s"));
    let z = TrafficLight::GREEN(String::from("30s"));

    x.print_time();
    y.print_time();
    z.print_time();

}
