use crate::uniform::UniformRandomGenerator;

pub trait DistributionConverter<G> {
    fn generate_from_uniform(&self, generator: &mut G) -> f64
        where G: UniformRandomGenerator;
}