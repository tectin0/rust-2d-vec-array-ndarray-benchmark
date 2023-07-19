use itertools::izip;
use test::Bencher;

use crate::{constants::SUB_INDICES, flattened_vec::make_flattened_vec};

fn make_vec_of_vecs(vec: Vec<f32>) -> Vec<Vec<f32>> {
    let mut vec_of_vecs: Vec<Vec<f32>> = Vec::new();

    for (x, y, z) in izip!(
        vec.iter().step_by(3),
        vec.iter().skip(1).step_by(3),
        vec.iter().skip(2).step_by(3)
    ) {
        vec_of_vecs.push(vec![*x, *y, *z]);
    }

    return vec_of_vecs;
}

fn add_one_to_each_element_vec_of_vecs(vec_of_vecs: &mut Vec<Vec<f32>>) {
    for vec in vec_of_vecs {
        for element in vec.iter_mut() {
            *element += 1.0;
        }
    }
}

fn add_one_to_x_value_vec_of_vecs(vec_of_vecs: &mut Vec<Vec<f32>>) {
    for vec in vec_of_vecs {
        vec[0] += 1.0;
    }
}

fn get_sub_vec_of_vecs(vec_of_vecs: &Vec<Vec<f32>>, sub_indices: [usize; 6]) -> Vec<Vec<f32>> {
    let mut sub_vec_of_vecs: Vec<Vec<f32>> = Vec::new();

    for index in sub_indices.iter() {
        sub_vec_of_vecs.push(vec_of_vecs[*index].clone());
    }

    return sub_vec_of_vecs;
}

#[bench]
fn benchmark_add_one_to_each_element_vec_of_vecs(b: &mut Bencher) {
    let mut vec_of_vecs = make_vec_of_vecs(make_flattened_vec());

    b.iter(|| add_one_to_each_element_vec_of_vecs(&mut vec_of_vecs));
}

#[bench]
fn benchmark_add_one_to_x_value_vec_of_vecs(b: &mut Bencher) {
    let mut vec_of_vecs = make_vec_of_vecs(make_flattened_vec());

    b.iter(|| add_one_to_x_value_vec_of_vecs(&mut vec_of_vecs));
}

#[bench]
fn benchmark_get_sub_vec_of_vecs(b: &mut Bencher) {
    let vec_of_vecs = make_vec_of_vecs(make_flattened_vec());

    b.iter(|| get_sub_vec_of_vecs(&vec_of_vecs, SUB_INDICES));
}
