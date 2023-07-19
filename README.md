# Rust Array Implementations Benchmarking
## Description

This repository serves as a benchmarking for different implementations of 2d arrays in Rust - specifically two dimensional arrays with the dimension of (n, 3) used to describe three-dimensional points in space. It compares the execution time of accessing and modifying two-dimensional array representations that include:
- Array2<f32> from the ndarray crate
- A flattened vector with the standard library Vec<f32>
- A vector of arrays Vec<[f32; 3]>
- A vector of vectors Vec<Vec<f32>>

## Results

The current test results for each array representation are outlined below:

| Test | Execution Time (ns/iter) |
|:-----|:-------------------------|
|benchmark_add_one_to_each_element_array2 | 102,033 ns/iter (+/- 27,931) |
|benchmark_add_one_to_x_value_array2 | 18,199 ns/iter (+/- 1,289) |
|benchmark_get_sub_array2 | 273 ns/iter (+/- 19) |
|benchmark_add_one_to_each_element_flattened_vec | 17,347 ns/iter (+/- 12,005) |
|benchmark_add_one_to_x_value_flattened_vec | 25,115 ns/iter (+/- 8,057) |
|benchmark_get_sub_flattened_vec | 4,249 ns/iter (+/- 8,985) |
|benchmark_add_one_to_each_element_vec_of_arrays | 40,618 ns/iter (+/- 29,715) |
|benchmark_add_one_to_x_value_vec_of_arrays | 21,157 ns/iter (+/- 24,437) |
|benchmark_get_sub_vec_of_arrays | 160 ns/iter (+/- 166) |
|benchmark_add_one_to_each_element_vec_of_vecs | 163,413 ns/iter (+/- 183,558) |
|benchmark_add_one_to_x_value_vec_of_vecs | 91,462 ns/iter (+/- 164,705) |
|benchmark_get_sub_vec_of_vecs | 606 ns/iter (+/- 402) |
