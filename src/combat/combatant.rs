extern crate rand;
use rand::seq::SliceRandom;

use std::time::SystemTime;

struct Maneuver {
    name: String,
    round_time: u64,
    damage: u32,
}

impl Maneuver {
    fn new(name: &str, round_time: u64, damage: u32) -> Maneuver {
        Maneuver{
            name: name.to_string(),
            round_time: round_time,
            damage: damage
        }
    }
}

pub struct Combatant {
    name: String,
    damage: u32,
    last_action: SystemTime,
    round_time: u64,
    maneuvers: Vec<Maneuver>
}

impl Combatant {
    pub fn new(name: &str, last_action: SystemTime) -> Combatant {
        let punch = Maneuver::new("Punch", 2, 1);
        let kick = Maneuver::new("Kick", 5, 3);
        let maneuvers = vec![punch, kick];
        Combatant {
            name: name.to_string(),
            damage: 0,
            round_time: 1,
            last_action: last_action,
            maneuvers: maneuvers
        }
    }

    pub fn print_status (&self){
        println!("{} has taken {} total damage.", self.name, self.damage)
    }

    pub fn take_damage (&mut self, damage: u32){
        self.damage = self.damage + damage
    }

    pub fn act(&mut self, time_now: SystemTime) {
        let time_since_last_action = time_now.duration_since(self.last_action)
           .expect("Clock may have gone backwards");
        if time_since_last_action.as_secs() >= self.round_time {
            self.perform_maneuver(time_now);
            println!("{:?}", time_since_last_action);
        }
    }

    fn perform_maneuver(&mut self, time_now: SystemTime) {
      let maneuver = self.maneuvers.choose(&mut rand::thread_rng()).unwrap();
      self.last_action = time_now;
      self.round_time = maneuver.round_time;
      println!("");
      println!("    {}: {}, [round time {}]", self.name, maneuver.name, self.round_time);
    }
}
