use std::time::SystemTime;

pub mod combatant;
use combatant::*;

pub struct Battle {
    pub combatants: Vec<Combatant>
}

impl Battle {
    pub fn new() -> Battle {
        let boot_time = SystemTime::now();
        let player_1 = Combatant::new("player_1", boot_time);
        let player_2 = Combatant::new("player_2", boot_time);
        let combatants = vec![player_1, player_2];
        Battle {
            combatants: combatants
        }
    }

    fn render (&self) {
        for combatant in &self.combatants {
            combatant.print_status();
        }
    }

    pub fn tick (&mut self) {
        let time_now = SystemTime::now();
        for combatant in &mut self.combatants {
            combatant.act(time_now)
        }
    }
}
