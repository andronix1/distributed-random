use crate::uniform::UniformRandomGenerator;

pub trait DistributionConverter<G>
    where G: UniformRandomGenerator
{
    fn generate_from_uniform(&self, generator: &mut G) -> f64;
}