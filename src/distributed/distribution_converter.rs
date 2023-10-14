use crate::uniform::UniformRandomGenerator;

pub trait DistributionConverter {
    fn generate_from_uniform<G>(&self, generator: &mut G) -> f64
        where G: UniformRandomGenerator;
}