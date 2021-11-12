#![allow(unused_variables)]
#![allow(dead_code)]

enum Device {
    Thermometer(Thermometer),
    Socket(Socket),
}

struct House {
    name: String,
    rooms: Vec<Room>,
}

trait IntoHouse {
    fn new(name: String) -> Self;
    fn get_room(&mut self, name: String) -> &mut Room;
    fn add_room(&self, room: Room) -> bool;
    fn remove_room(&self, name: String) -> bool;
    fn list_room(&self) -> Vec<Room>;
    fn report_devices(&self);
}

struct Room {
    name: String,
    devices: Vec<Device>,
}

trait IntoRoom {
    fn new(name: String) -> Self;
    fn add_device(&self, device: Device) -> bool;
    fn remove_device(&self, name: String) -> bool;
}

struct Socket {
    name: String,
    description: String,
    watts: usize,
}

trait IntoSocket {
    fn new_device(name: String, description: String) -> Device;
    fn off(&self) -> bool;
    fn on(&self) -> bool;
    fn get_watts(&self) -> usize;
}

struct Thermometer {
    name: String,
    description: String,
    temperature: usize,
}

trait IntoThermometer {
    fn new_device(name: String, description: String) -> Device;
    fn get_temperature(&self) -> usize;
}

fn main() {
    println!("Прототип умный дом")
}
