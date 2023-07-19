use ndarray::{Array2, Axis};
use test::Bencher;

use crate::{constants::SUB_INDICES, flattened_vec::make_flattened_vec};

fn make_array2_from_flattened_vec(vec: Vec<f32>) -> Array2<f32> {
    let shape = (vec.len() / 3, 3);

    let array: Array2<f32> = Array2::from_shape_vec(shape, vec).unwrap();

    return array;
}

fn add_one_to_each_element_array2(array: &mut Array2<f32>) {
    for row in array.axis_iter_mut(ndarray::Axis(0)) {
        for element in row {
            *element += 1.0;
        }
    }
}

fn add_one_to_x_value_array2(array: &mut Array2<f32>) {
    for mut row in array.axis_iter_mut(ndarray::Axis(0)) {
        row[0] += 1.0;
    }
}

fn get_sub_array2(array: &Array2<f32>, sub_indices: [usize; 6]) -> Array2<f32> {
    let sub_array = array.select(Axis(0), &sub_indices);

    return sub_array;
}

#[bench]
fn benchmark_add_one_to_each_element_array2(b: &mut Bencher) {
    let mut array = make_array2_from_flattened_vec(make_flattened_vec());

    b.iter(|| add_one_to_each_element_array2(&mut array));
}

#[bench]
fn benchmark_add_one_to_x_value_array2(b: &mut Bencher) {
    let mut array = make_array2_from_flattened_vec(make_flattened_vec());

    b.iter(|| add_one_to_x_value_array2(&mut array));
}

#[bench]
fn benchmark_get_sub_array2(b: &mut Bencher) {
    let array = make_array2_from_flattened_vec(make_flattened_vec());

    b.iter(|| get_sub_array2(&array, SUB_INDICES));
}
