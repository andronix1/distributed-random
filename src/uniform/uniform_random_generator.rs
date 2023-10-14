pub trait UniformRandomGenerator {
    fn next(&mut self) -> f64;
}