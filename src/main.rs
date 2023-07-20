use std::fmt;
use std::ops::Add;
#[derive(PartialEq)]
struct Clock {
    hour: u32,
    minute: u32,
}

impl Clock {
    fn new(hour: u32, minute: u32) -> Self {
        if hour >= 24 || minute >= 60 {
            panic!("Invalid time...");
        }
        Self {
            hour,
            minute,
        }
    }
    
    fn add_minutes(&mut self, val: u32){
        let minute = self.minute + val;
        self.hour += minute / 60 as u32;
        self.hour %= 24;
        self.minute = minute % 60;
    }
}

impl Add for Clock {
    type Output = Clock;

    fn add(self, other: Clock) -> Clock {
        let minutes: u32 = self.minute + other.minute;
        Clock {
            hour: (self.hour + other.hour + minutes / 60) % 24,
            minute: minutes % 60,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{hour:0>2} : {minutes:0>2}", hour = self.hour, minutes = self.minute)
    }
}
fn main() {
    let mut p1 = Clock::new(11, 5);
    let mut p2 = Clock::new(4, 10);

    p1.add_minutes(812);

    p2.add_minutes(14);
    let s = p1 + p2;
    println!("The time is : {}", s);
}
