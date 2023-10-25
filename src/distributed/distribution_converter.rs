use crate::uniform::UniformRandomGenerator;

pub trait DistributionConverter {
    fn generate_from_uniform(&self, generator: &mut Box<dyn UniformRandomGenerator>) -> f64;
}