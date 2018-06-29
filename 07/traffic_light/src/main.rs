#![allow(unused_variables)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};
use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
