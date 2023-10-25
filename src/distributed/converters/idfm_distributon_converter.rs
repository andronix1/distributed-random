use crate::{uniform::UniformRandomGenerator, distributed::DistributionConverter};

pub struct IdfmDistributionConverter {
    pub inverse_distribution: fn(f64) -> f64,
}

impl IdfmDistributionConverter {
    pub fn new(inverse_distribution: fn(f64) -> f64) -> Self {
        Self {
            inverse_distribution,
        }
    }
}

impl DistributionConverter for IdfmDistributionConverter {
    fn generate_from_uniform(&self, uniform_generator: &mut Box<dyn UniformRandomGenerator>) -> f64 {
        (self.inverse_distribution)(uniform_generator.next())
    }
}