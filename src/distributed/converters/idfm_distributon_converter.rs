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

impl<G> DistributionConverter<G> for IdfmDistributionConverter
    where G: UniformRandomGenerator 
{
    fn generate_from_uniform(&self, uniform_generator: &mut G) -> f64
        where G: UniformRandomGenerator 
    {
        (self.inverse_distribution)(uniform_generator.next())
    }
}