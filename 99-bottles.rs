use std::fmt;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Beers(i32);

struct Wall {
    max: Beers,
    current: Beers,
}

impl Wall {
    fn from(n: i32) -> Self {
        Self { max: Beers(n), current: Beers(n) }
    }

    fn next(&mut self) -> Beers {
        self.current = Beers((self.current.0 - 1).rem_euclid(self.max.0 + 1));
        self.current
    }
}


impl fmt::Display for Beers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            n if n > 1 => write!(f, "{} bottles", n),
            n if n == 1 => write!(f, "{} bottle", n),
            _ => write!(f, "no more bottles"),
        }
    }
}

fn main() {
    let arg = std::env::args().last();
    
    let mut wall = Wall::from(3);
    loop {
        let current = wall.current;
        let next = wall.next();
        let action = if current == Beers(0) { "Go to the store and buy some more" } else { "Take one down and pass it around" };
        println!("\n{current} of beer on the wall, {current} of beer.\n{action}, {next} of beer on the wall.");
    
        if arg == Some("break".to_string()) && next == wall.max {
            break
        }
    }
}
