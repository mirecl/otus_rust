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

impl House {
    fn new() -> Self {
        todo!()
    }
    fn get_room(name: String) -> Room {
        todo!()
    }
    fn add_room(room: Room) -> bool {
        todo!()
    }
    fn remove_room(name: String) -> bool {
        todo!()
    }
    fn list_room() -> Vec<Room> {
        todo!()
    }
    fn report_devices() {
        todo!()
    }
}

struct Room {
    name: String,
    devices: Vec<Device>,
}

impl Room {
    fn new(name: String) -> Self {
        todo!()
    }
    fn add_device(device: Device) -> bool {
        todo!()
    }
    fn remove_device(name: String) -> bool {
        todo!()
    }
}

struct Socket {
    name: String,
    description: String,
    power: usize,
}

impl Socket {
    fn new_device(name: String, description: String) -> Device {
        todo!()
    }
    fn off(&self) {
        todo!()
    }
    fn on(&self) {
        todo!();
    }
    fn get_current_power(&self) -> usize {
        todo!()
    }
}

struct Thermometer {
    name: String,
    description: String,
    temperature: usize,
}

impl Thermometer {
    fn new_device(name: String, description: String) -> Device {
        todo!()
    }
    fn get_temperature(&self) -> usize {
        todo!();
    }
}

fn main() {
    println!("Прототип умный дом")
}
