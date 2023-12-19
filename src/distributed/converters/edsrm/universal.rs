use crate::{distributed::DistributionConverter, uniform::UniformRandomGenerator};

use super::EdsrmMonotousDistributionConverter;

pub type Ranges = Vec<f64>;

struct ConverterData {
    converter: EdsrmMonotousDistributionConverter,
    area_before: f64
}

pub struct EdsrmUniversalDistributionConverter {
    converters: Vec<ConverterData>,
    area: f64
}

impl EdsrmUniversalDistributionConverter {
    pub fn new(ranges: Ranges, distribution: fn(f64) -> f64, majorant_size_per_range: usize) -> Result<EdsrmUniversalDistributionConverter, ()> {
        if ranges.len() < 2 {
            return Err(())
        }
        let mut result = Vec::with_capacity(ranges.len() - 1);

        let mut area = 0.0;
        for i in 0..ranges.len() - 1 {
            let converter = EdsrmMonotousDistributionConverter::new(
                distribution, 
                ranges[i], 
                ranges[i + 1], 
                majorant_size_per_range
            )?;
            area += converter.get_majorant().get_area();
            result.push(ConverterData { area_before: area, converter });
        }

        return Ok(EdsrmUniversalDistributionConverter {
            area,
            converters: result,
        });
    }
}

impl<G> DistributionConverter<G> for EdsrmUniversalDistributionConverter 
    where G: UniformRandomGenerator
{
    fn generate_from_uniform(&self, generator: &mut G) -> f64 
        where G: UniformRandomGenerator
    {
        loop {
            let gen = generator.next();
            let position = gen * self.area;
            let mut id = 0;
            for i in 0..self.converters.len() {
                if position < self.converters[i].area_before {
                    id = i;
                    break;
                }
            }
            let gen1 = if id == 0 {
                gen * self.converters[id].area_before
            } else {
                let area_before_previous = self.converters[id - 1].area_before;
                let area_before = self.converters[id].area_before;
                gen * (area_before - area_before_previous) + area_before_previous
            };
            if let Some(result) = self.converters[id].converter.try_generate_from_uniform_in_range(gen1, generator) {
                return result;
            }
        }
    }
}