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
    fn get_room(&mut self, name: String) -> &mut Room {
        todo!()
    }
    fn add_room(&self, room: Room) -> bool {
        todo!()
    }
    fn remove_room(&self, name: String) -> bool {
        todo!()
    }
    fn list_room(&self) -> Vec<Room> {
        todo!()
    }
    fn report_devices(&self) {
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
    fn add_device(&self, device: Device) -> bool {
        todo!()
    }
    fn remove_device(&self, name: String) -> bool {
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
