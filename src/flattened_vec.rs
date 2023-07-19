use itertools::iproduct;
use test::Bencher;

use crate::constants::SUB_INDICES;

pub(crate) fn make_flattened_vec() -> Vec<f32> {
    let x_min = 0.8;
    let x_max = 1.1;
    let y_min = -0.1;
    let y_max = 0.1;
    let z_min = -1.0;
    let z_max = 1.0;

    let x_steps = 10;
    let y_steps = 20;
    let z_steps = 250;

    let mut cartesian_geometry: Vec<f32> = Vec::new();

    for (x_step, y_step, z_step) in iproduct!(0..x_steps, 0..y_steps, 0..z_steps) {
        let x = x_min + (x_max - x_min) * (x_step as f32) / (x_steps as f32);
        let y = y_min + (y_max - y_min) * (y_step as f32) / (y_steps as f32);
        let z = z_min + (z_max - z_min) * (z_step as f32) / (z_steps as f32);

        cartesian_geometry.push(x);
        cartesian_geometry.push(y);
        cartesian_geometry.push(z);
    }

    println!("Geometry size: ({:?}, 3)", cartesian_geometry.len() / 3);

    return cartesian_geometry;
}

fn add_one_to_each_element_flattened_vec(vec: &mut Vec<f32>) {
    for i in 0..vec.len() {
        vec[i] += 1.0;
    }
}

fn add_one_to_x_value_flattened_vec(vec: &mut Vec<f32>) {
    for i in (0..vec.len()).step_by(3) {
        vec[i] += 1.0;
    }
}

fn get_sub_flattened_vec(vec: &Vec<f32>, sub_indices: [usize; 6]) -> Vec<f32> {
    let mut sub_flattened_vec: Vec<f32> = Vec::new();

    for i in sub_indices.iter() {
        let index = *i * 3;

        sub_flattened_vec.push(vec[index]);
        sub_flattened_vec.push(vec[index + 1]);
        sub_flattened_vec.push(vec[index + 2]);
    }

    return sub_flattened_vec;
}

#[bench]
fn benchmark_add_one_to_each_element_flattened_vec(b: &mut Bencher) {
    let mut vec = make_flattened_vec();

    b.iter(|| add_one_to_each_element_flattened_vec(&mut vec));
}

#[bench]
fn benchmark_add_one_to_x_value_flattened_vec(b: &mut Bencher) {
    let mut vec = make_flattened_vec();

    b.iter(|| add_one_to_x_value_flattened_vec(&mut vec));
}

#[bench]
fn benchmark_get_sub_flattened_vec(b: &mut Bencher) {
    let vec = make_flattened_vec();

    b.iter(|| get_sub_flattened_vec(&vec, SUB_INDICES));
}
