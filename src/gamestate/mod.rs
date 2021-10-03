use raylib::prelude::*;

enum Class {
    Triggerman,
    RunAndGun,
}

struct Player {
    pos: Vector3,
    rot: Quaternion,
    class: Class,
}

struct Time {
    minutes: u32,
    seconds: u32,
    milliseconds: u8,
}

impl Time {
    pub fn from(s: String) -> Self {
        let mut v = s.split(":").collect::<Vec<_>>();
        let a = v[1].split(".").collect::<Vec<_>>();
        v[1] = a[0];
        v.push(a[1]);
        Self {
            minutes: v[0].parse().unwrap(),
            seconds: v[1].parse().unwrap(),
            milliseconds: v[1].parse().unwrap(),
        }
    }
}

struct Gamestate {
    messages: Vec<String>,
    players: Vec<Player>,
    time: Time,
}
