use super::uniform_random_generator::UniformRandomGenerator;

pub struct MultiplicativeRandomGenerator {
    a1: u64,
    a2: u64,
    a3: u64,
}

impl MultiplicativeRandomGenerator {
    pub fn new() -> Self {
        MultiplicativeRandomGenerator {
            a1: 1,
            a2: 0,
            a3: 0
        }
    }
}

impl UniformRandomGenerator for MultiplicativeRandomGenerator {
    fn next(&mut self) -> f64 {
        let c = 11973 * self.a1;
        let d = (2800 * self.a1) + (11973 * self.a2) + (c >> 14);
        self.a1 = c << 50 >> 50;
        self.a2 = d << 50 >> 50;
        self.a3 = ((2842 * self.a1) + (2800 * self.a2) + (11973 * self.a3) + (d >> 14)) << 50 >> 50;
        return (self.a3 as f64 + (self.a2 as f64 + self.a1 as f64 / 16384.0) / 16384.0) / 4096.0 / 4.0;
    }
}