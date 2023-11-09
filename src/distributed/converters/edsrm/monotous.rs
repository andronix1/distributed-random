use crate::{
    distributed::DistributionConverter, uniform::UniformRandomGenerator, utils::get_optimal_value,
};

struct MajorantColumn {
    x: f64,
    inner_height: f64,
    height: f64,
    width: f64,
}
pub struct Majorant {
    columns: Vec<MajorantColumn>,
}

impl Majorant {
    fn new(distribution: fn(f64) -> f64, start: f64, end: f64, size: usize) -> Option<Self> {
        let is_rising = distribution(start) < distribution(end);
        if let Some(delta_x) =
            Majorant::get_optimal_delta_x(start, end, is_rising, size, distribution)
        {
            return Some(Majorant {
                columns: Majorant::get_majorant_columns_from_initial_delta_x(
                    delta_x,
                    is_rising,
                    distribution,
                    start,
                    end,
                    size,
                ),
            });
        }
        return None;
    }

    fn overflowed(
        initial_delta_x: f64,
        distribution: fn(f64) -> f64,
        is_rising: bool,
        start: f64,
        end: f64,
        size: usize,
    ) -> bool {
        let start_pos = if is_rising { end } else { start };
        let area: f64 = initial_delta_x * (distribution)(start_pos).abs();
        let mut x: f64 = start_pos;
        for _ in 0..size {
            let delta_x: f64 = area / (distribution)(x).abs();
            x += if is_rising { -delta_x } else { delta_x };
            if x < start || x > end {
                return true;
            }
        }
        return false;
    }

    fn get_majorant_columns_from_initial_delta_x(
        initial_delta_x: f64,
        is_rising: bool,
        distribution: fn(f64) -> f64,
        start: f64,
        end: f64,
        size: usize,
    ) -> Vec<MajorantColumn> {
        let mut result = Vec::new();
        result.reserve_exact(size);
        let start_pos = if is_rising { end } else { start };
        let area: f64 = initial_delta_x * (distribution)(start_pos).abs();
        let mut x: f64 = start_pos;
        for _ in 0..size {
            let height = (distribution)(x);
            let delta_x: f64 = area / height.abs();

            let mut start_x = x;
            if is_rising {
                start_x -= delta_x;
            }

            x += if is_rising { -delta_x } else { delta_x };
            result.push(MajorantColumn {
                x: start_x,
                inner_height: (distribution)(x),
                height,
                width: delta_x,
            });
        }
        return result;
    }

    fn get_optimal_delta_x(
        start: f64,
        end: f64,
        is_rising: bool,
        size: usize,
        distribution: fn(f64) -> f64,
    ) -> Option<f64> {
        get_optimal_value((end - start) / size as f64, |x| {
            Majorant::overflowed(x, distribution, is_rising, start, end, size)
        })
    }

    pub fn get_area(&self) -> f64 {
        let column = &self.columns[0];
        return column.width * column.height * self.columns.len() as f64;
    }
}

pub struct EdsrmMonotousDistributionConverter {
    distribution: fn(f64) -> f64,
    majorant: Majorant,
}

impl EdsrmMonotousDistributionConverter {
    pub fn new(
        distribution: fn(f64) -> f64,
        start: f64,
        end: f64,
        majorant_size: usize,
    ) -> Result<Self, ()> {
        Ok(EdsrmMonotousDistributionConverter {
            distribution,
            majorant: Majorant::new(distribution, start, end, majorant_size).ok_or(())?,
        })
    }

    pub fn get_majorant(&self) -> &Majorant {
        return &self.majorant;
    }
}

impl<G> DistributionConverter<G> for EdsrmMonotousDistributionConverter
where
    G: UniformRandomGenerator,
{
    fn generate_from_uniform(&self, generator: &mut G) -> f64
    where
        G: UniformRandomGenerator,
    {
        loop {
            let x_gen: f64 = generator.next();
            let column_index: usize = (x_gen * self.majorant.columns.len() as f64) as usize;
            let column = &self.majorant.columns[column_index];
            let y: f64 = generator.next() * column.height;
            let result = column.x
                + column.width * (self.majorant.columns.len() as f64 * x_gen - column_index as f64);
            if y <= column.inner_height || y <= (self.distribution)(result) {
                return result;
            }
        }
    }
}

