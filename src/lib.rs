pub mod uniform;
pub mod distributed;
mod utils;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{distributed::{converters::edsrm::EdsrmMonotousDistributionConverter, DistributionConverter}, uniform::MultiplicativeRandomGenerator};

    #[test]
    pub fn monotous_speed_check() {
        let mut gen = MultiplicativeRandomGenerator::new();
        let con = EdsrmMonotousDistributionConverter::new(
            |x| 2.0 * x,
            0.0, 1.0,
            330
        ).unwrap();
        let count = 10000000;
        let start = Instant::now();
        for _ in 0..count {
            let _ = con.generate_from_uniform(&mut gen);
        }
        println!("{:?}", start.elapsed() / count)
    }
}