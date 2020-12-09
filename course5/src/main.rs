use std::f64::consts::PI;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
enum TrafficSignals {
    Red, 
    Green, 
    Yellow,
}

pub trait Time {
    fn get_time(&self) -> u32 {
        0
    }
}

impl Time for TrafficSignals {
    fn get_time(&self) -> u32 {
        match *self {
            TrafficSignals::Red => 10,
            TrafficSignals::Green => 15,
            TrafficSignals::Yellow => 3,
        }
    }
}

#[test]
fn test_traffic_sinals() {
    let signals = TrafficSignals::Red;
    println!("time = {}", signals.get_time());

    let signals = TrafficSignals::Green;
    println!("time = {}",signals.get_time());

    let signals = TrafficSignals::Yellow;
    println!("time = {}", signals.get_time());
}

fn sum_u32(arr: &[u32]) -> Option<u32> {
    let mut temp_sum : u32 = 0;

   for val in arr.iter() {
    let ret = temp_sum.checked_add(*val);
    match ret { 
        Some(val) => { 
            temp_sum = val;
        },
        None => return None,
    };
   }

   Some(temp_sum)
}

#[test] 
fn test_sum_u32() {
    let arr = vec![1000000000; 32];
    let ret = sum_u32(&arr);
    println!("ret = {:?}", ret);
}

trait Area {
    fn area(&self) -> f64 {
        0.
    }
}
fn display_area<T: Area>(para: T) {
    println!("area is {}", para.area());
}

struct Circle {
    radius: f64,
}
impl  Circle { 
    pub fn new(radius: f64 ) -> Self {
        Self {
            radius
        }
    }
}
impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

struct Trangle {
    width: f64,
    height: f64,
}

impl  Trangle{ 
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
        }
    }
}

impl  Area for Trangle {
    fn area(&self) -> f64 {
        self.width * self.height / 2.0
    }
}

struct Square {
    width: f64,
}

impl Square {
    pub fn new( width: f64) -> Self {
        Self {
            width
        }
    }
}
impl  Area for Square {
    fn area(&self) -> f64 {
        self.width * self.width
    }
}

#[test]
fn test_area() {
    let circle = Circle::new(1.0f64);
    display_area(circle);
    let trangle = Trangle::new(1.0, 2.0);
    display_area(trangle);
    let square = Square::new(2.0);
    display_area(square);

}