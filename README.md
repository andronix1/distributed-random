# Distributed random
## О библиотеке
Библиотека позволяет генерировать случайные числа с указанной плотностью распределения 
## Установка
Для того, чтобы использовать библиотеку, добавьте зависимость в файл `Cargo.toml`
```toml
[dependencies]
distributed_random = { git = "https://github.com/andronix1/distributed-random" }
```
## Библиотека
### Модуль uniform
```rust
pub trait UniformRandomGenerator {
    fn next(&mut self) -> f64;
}
```
`UniformRandomGenerator` описывает генератор равномерно распределённых случайных чисел в промежутке (0; 1). Реализован мультипликативный генератор метода вычетов `MultiplicativeRandomGenerator`.
```rust
let mut rand_generator = MultiplicativeRandomGenerator::new();
let random_value = generator.next();
```
### Модуль distributed
```rust
pub trait DistributionConverter {
    fn generate_from_uniform<G>(&self, generator: &mut G) -> f64
        where G: UniformRandomGenerator;
}
```
`DistributionConverter` описывает генератор случайных чисел с заданым распределением на основе равномерной случайной величины из `UniformRandomGenerator`. 
#### IdfmDistributionConverter - Inverse Distribution Function Method
Реализует метод обратной функции распределения. Для создания нужно указать обратную функцию распределения.
```rust
// Создаём конвертер с плотностью распределения f(x) = 2.0 * x. Обратной функцией распределения для f(x) является Fi(x) = sqrt(x)
let converter = IdfmDistributionConverter::new(
    |x| x.sqrt()
);
let distributed_random_value = converter.generate_from_uniform(&mut rand_generator);
```
#### edsrm - Economical Double-Sided Rejection Method
##### EdsrmMonotousDistributionConverter
Реализует двусторонний метод исключения для монотонных плотностей распределения. Для создания нужно указать функцию плотности распределения, промежуток из функции, размер(точность) сетки.
```rust
// Создаём конвертер с плотностью распределения f(x) = 2.0 * x
let converter = EdsrmMonotousDistributionConverter::new(
    |x| 2.0 * x, // distribution
    0.0, 1.0,    // start, end 
    330          // majorant_size
).unwrap(); 
// Ошибка возникает, если функция не удалось построить мажоранту(плотность распределения на промежутке должна быть монотонной, положительной, непрерывной и ограниченной)
let distributed_random_value = converter.generate_from_uniform(&mut rand_generator);
```

##### EdsrmUniversalDistributionConverter
Реализует двусторонний метод исключения для плотностей распределения с любым количеством кусков монотонности. Для создания нужно указать промежутки монотонности, функцию плотности распределения, размер(точность) сетки для каждого промежутка.
```rust
// Создаём конвертер с плотностью распределения f(x) = 2.0 * x
let converter = EdsrmUniversalDistributionConverter::new(
    vec![0.0, 0.3, 0.6, 1.0],    // ranges - (0.0, 0.3) + (0.3, 0.6) + (0.6, 1.0) 
    |x| 2.0 * x,                 // distribution
    330                          // majorant_size_per_range
).unwrap(); 
// Ошибка возникает, если функция не удалось построить мажоранту на хотя бы одном из промежутков монотонности(плотность распределения на промежутке должна быть положительной, непрерывной и ограниченной)
let distributed_random_value = converter.generate_from_uniform(&mut rand_generator);
```
