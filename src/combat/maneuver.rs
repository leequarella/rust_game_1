pub struct Maneuver {
    name: String,
    round_time: u64,
}

impl Maneuver {
    pub fn new(name: String, round_time: u64) {
        Maneuver{
            name: name,
            round_time: round_time
        }
    }
}
