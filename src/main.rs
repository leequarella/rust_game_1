use std::time::SystemTime;

mod combat;
use combat::*;


fn main() {
    let mut battle = Battle::new();

    let mut last_tick = SystemTime::now();

    while true {
        let time_now = SystemTime::now();
        let delta_time = time_now.duration_since(last_tick)
            .expect("Clock may have gone backwards");
        if delta_time.as_millis() >= 1000 {
            last_tick = time_now;
            println!("*");
        }

        battle.tick();
    }
}
