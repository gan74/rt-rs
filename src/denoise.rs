use crate::vec::*;
use crate::image::*;
use crate::color::*;
use crate::utils::*;


const SIGMA: f32 = 0.4;

pub fn denoise(input: &Image) -> Image {
    let mut pixels = vec![Color::from(0.0); input.pixel_count()];

    for x in 0..input.width() {
        for y in 0..input.height() {
            let index = input.pixel_index(x, y);
            pixels[index] = denoise_pixel(x, y, input, SIGMA);
        }
    }

    Image::new(input.width(), input.height(), pixels)
}


const NEIGH_SIZE: i32 = 3;
const NEIGH_SIDE_PX: usize = (NEIGH_SIZE * 2 + 1) as usize;
const NEIGH_PIXELS: usize = NEIGH_SIDE_PX * NEIGH_SIDE_PX;

const WEIGHT_SIZE: i32 = 10;
const WEIGHT_SIDE_PX: usize = (WEIGHT_SIZE * 2 + 1) as usize;
const WEIGHT_PIXELS: usize = WEIGHT_SIDE_PX * WEIGHT_SIDE_PX;

fn collect_neighbourhood(x: u32, y: u32, input: &Image) -> [Color; NEIGH_PIXELS] {
    let mut neighbourhood = [Color::from(0.0); NEIGH_PIXELS];

    let (x, y) = (x as i32, y as i32);
    for i in 0..NEIGH_PIXELS {
        let a = (i % NEIGH_SIDE_PX) as i32;
        let b = (i / NEIGH_SIDE_PX) as i32;
        neighbourhood[i] = input.pixel_at_clamped(x + a - NEIGH_SIZE, y + b - NEIGH_SIZE);
    }

    neighbourhood
}

fn compute_weights(x: u32, y: u32, neighbourhood: [Color; NEIGH_PIXELS], input: &Image, h: f32) -> [f32; WEIGHT_PIXELS] {
    let mut weights = [0.0; WEIGHT_PIXELS];
    let mut w_index = 0;

    let (x, y) = (x as i32, y as i32);
    for kx in -WEIGHT_SIZE..(WEIGHT_SIZE + 1) {
        for ky in -WEIGHT_SIZE..(WEIGHT_SIZE + 1) {
            let mut d2 = 0.0;
            let mut index = 0;

            for px in -NEIGH_SIZE..(NEIGH_SIZE + 1) {
                for py in -NEIGH_SIZE..(NEIGH_SIZE + 1) {
                    let qc = Vec3::from(input.pixel_at_clamped(x + kx + px, y + ky + py));
                    let pc = Vec3::from(neighbourhood[index]);

                    let diff = pc - qc;
                    let err = diff * diff;
                    d2 += err.x + err.y + err.z;

                    index += 1;
                }
            }

            weights[w_index] = (-d2 / (index as f32 * h * h)).exp();
            w_index += 1;
        }
    }

    weights
}

fn apply_weights(x: u32, y: u32, weights: [f32; WEIGHT_PIXELS], input: &Image) -> Color {
    let mut acc = Color::from(0.0);
    let mut total = 0.0;
    let mut max: f32 = 0.0;
    let mut w_index = 0;

    let center = input.pixel_at(x, y);

    let (x, y) = (x as i32, y as i32);
    for kx in -WEIGHT_SIZE..(WEIGHT_SIZE + 1) {
        for ky in -WEIGHT_SIZE..(WEIGHT_SIZE + 1) {
            let c = input.pixel_at_clamped(x + kx, y + ky);
            let w = weights[w_index];

            total += w;
            acc += c * w;

            max = max.max(w);
            w_index += 1;
        }
    }

    if max < EPSILON {
        center
    } else {
        acc += center * max;
        total += max;
        acc / total
    }
}

fn denoise_pixel(x: u32, y: u32, input: &Image, sigma: f32) -> Color {
    let neighbourhood = collect_neighbourhood(x, y, input);
    let weights = compute_weights(x, y, neighbourhood, input, sigma);

    apply_weights(x, y, weights, input)
}

