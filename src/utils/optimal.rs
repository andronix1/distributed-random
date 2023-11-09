pub fn get_optimal_value<F>(initial: f64, check: F) -> Option<f64> 
    where F: Fn(f64) -> bool
{
    let mut result: f64 = initial;
    let mut delta = 1.0;
    while delta >= result {
        delta /= 10.0;
    }
    while check(result) {
        if result - delta < 0.0 {
            delta /= 10.0;
        }
        result -= delta;
    }
    let mut one_digit_times = 0;
    loop {
        if check(result + delta) {
            delta /= 10.0;
            if delta < 0.1_f64.powi(f64::DIGITS as i32) {
                return Some(result);
            }
            one_digit_times = 0;
        } else {
            one_digit_times += 1;
            if one_digit_times > 10 {
                println!("{}", delta);
                return None;
            }
            result += delta;
        }
    }
}