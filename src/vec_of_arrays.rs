use itertools::izip;
use test::Bencher;

use crate::{constants::SUB_INDICES, flattened_vec::make_flattened_vec};

fn make_vec_of_arrays(vec: Vec<f32>) -> Vec<[f32; 3]> {
    let mut vec_of_arrays: Vec<[f32; 3]> = Vec::new();

    for (x, y, z) in izip!(
        vec.iter().step_by(3),
        vec.iter().skip(1).step_by(3),
        vec.iter().skip(2).step_by(3)
    ) {
        vec_of_arrays.push([*x, *y, *z]);
    }

    return vec_of_arrays;
}

fn add_one_to_each_element_vec_of_arrays(vec_of_arrays: &mut Vec<[f32; 3]>) {
    for array in vec_of_arrays {
        for element in array.iter_mut() {
            *element += 1.0;
        }
    }
}

fn add_one_to_x_value_vec_of_arrays(vec_of_arrays: &mut Vec<[f32; 3]>) {
    for array in vec_of_arrays {
        array[0] += 1.0;
    }
}

fn get_sub_vec_of_arrays(vec_of_arrays: &Vec<[f32; 3]>, sub_indices: [usize; 6]) -> Vec<[f32; 3]> {
    let mut sub_vec_of_arrays: Vec<[f32; 3]> = Vec::new();

    for index in sub_indices.iter() {
        sub_vec_of_arrays.push(vec_of_arrays[*index]);
    }

    return sub_vec_of_arrays;
}

#[bench]
fn benchmark_add_one_to_each_element_vec_of_arrays(b: &mut Bencher) {
    let mut vec_of_arrays = make_vec_of_arrays(make_flattened_vec());

    b.iter(|| add_one_to_each_element_vec_of_arrays(&mut vec_of_arrays));
}

#[bench]
fn benchmark_add_one_to_x_value_vec_of_arrays(b: &mut Bencher) {
    let mut vec_of_arrays = make_vec_of_arrays(make_flattened_vec());

    b.iter(|| add_one_to_x_value_vec_of_arrays(&mut vec_of_arrays));
}

#[bench]
fn benchmark_get_sub_vec_of_arrays(b: &mut Bencher) {
    let vec_of_arrays = make_vec_of_arrays(make_flattened_vec());

    b.iter(|| get_sub_vec_of_arrays(&vec_of_arrays, SUB_INDICES));
}
