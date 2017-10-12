use std::fmt;

struct Bus {
    name: &'static str,
}

struct Train {
    line: &'static str,
}

struct Plane {
    callsign: &'static str,
}

impl fmt::Display for Bus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} bus", self.name)
    }
}
impl fmt::Display for Train {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} train", self.line)
    }
}
impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} flight", self.callsign)
    }
}

fn main() {
    let b1 = Bus { name: "LAX Flyaway" };
    let t1 = Train { line: "Pacific Coastliner"};
    let p1 = Plane { callsign: "El Mariachi"};

    println!("I took the {} to the {} and then hopped on the {}.", b1, t1, p1); 
}
