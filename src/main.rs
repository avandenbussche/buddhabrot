pub mod complex;
pub mod settings;

extern crate image;
extern crate rayon;

use std::env;
use std::path;
use std::time;
use rayon::prelude::*;

fn main() {
	println!("\nWelcome to the buddhabrot generator!\n");

	let default_render_settings = settings::RenderSettings {
		x_min: -2.0,
		x_max: 0.5,
		y_min: -1.25,
		y_max: 1.25,
		max_iterations: 80,
		output_width: 1920,
		output_dir: format!("{}/", get_home_directory()),
	};

	let user_render_settings = settings::prompt_render_settings(&default_render_settings);

	// for timing
	let start_time = time::Instant::now();

	// mandelbrot set demo
	let num_pixels = user_render_settings.output_width * user_render_settings.get_height();
	let resolution_x = user_render_settings.get_resolution_x();
	let resolution_y = user_render_settings.get_resolution_y();
	let mut current_x: f64;
	let mut current_y = user_render_settings.y_max;

	let mut converged = false;
	let mut z: complex::Complex;
	
	let mut num_iterations_until_covergence: Vec<u32> = Vec::with_capacity(num_pixels as usize);
	let mut most_iterations_required: u32 = 0;

	let mut par_num_iterations_until_covergence: Vec<u32> = Vec::with_capacity(num_pixels as usize);
	let mut par_most_iterations_required: u32 = 0;

	// start in top left corner and move first across cols, then down rows
	// for _ in 0..user_render_settings.get_height() {
	// 	current_x = user_render_settings.x_min;

	// 	for _ in 0..user_render_settings.output_width {
	// 		z = complex::Complex(0.0, 0.0);

	// 		for i in 1..user_render_settings.max_iterations {
	// 			z.squared();
	// 			z.add(complex::Complex(current_x, current_y));

	// 			// if the modulus ever grows larger than 2, it will never converge
	// 			if z.norm_squared() > 4.0 {
					
	// 				if i > most_iterations_required {
	// 					most_iterations_required = i;
	// 				}

	// 				num_iterations_until_covergence.push(i);
	// 				converged = true;
	// 				break;

	// 			}
	// 		}

	// 		if !converged {
	// 			num_iterations_until_covergence.push(0);
	// 		}

	// 		current_x += resolution_x;
	// 		converged = false;
	// 	}

	// 	current_y -= resolution_y;
	// }

	let par_num_iterations_until_covergence_iter = (0..num_pixels).into_par_iter().map(|pixel_i| {

		let current_pixel_x = pixel_i % user_render_settings.output_width;
		let current_pixel_y = pixel_i / user_render_settings.output_width;

		let resolution_x = user_render_settings.get_resolution_x();
		let resolution_y = user_render_settings.get_resolution_y();

		let current_x = user_render_settings.x_min + current_pixel_x as f64 * resolution_x;
		let current_y = user_render_settings.y_min + current_pixel_y as f64 * resolution_y;

		let mut z = complex::Complex(0.0, 0.0);

		for i in 1..user_render_settings.max_iterations {

			z.squared();
			z.add(complex::Complex(current_x, current_y));

			// if the modulus ever grows larger than 2, it will never converge
			if z.norm_squared() > 4.0 {
				return i;
			}
		}

		return 0;

	});

	let par_num_iterations_until_covergence: Vec<u32> = par_num_iterations_until_covergence_iter.collect();
	
	// convert num iterations to colors
	let mut image_buffer: image::ImageBuffer<image::Rgb<u8>,_> = image::ImageBuffer::new(user_render_settings.output_width,
																						 user_render_settings.get_height());

	let mut i = 0;
	for current_pixel_y in 0..user_render_settings.get_height() { 
		for current_pixel_x in 0..user_render_settings.output_width {

			let g = remap(par_num_iterations_until_covergence[i] as f64, (0.0, user_render_settings.max_iterations as f64), (0.0, 255.0)).round() as u8;

			let this_pixel = image_buffer.get_pixel_mut(current_pixel_x, current_pixel_y);
			let image::Rgb(_data) = *this_pixel;
			*this_pixel = image::Rgb([g, g, g]);

			i += 1;
		}
	}

	// render output
	let save_path_string = format!(
		"{}buddhabrot_{:?}.png",
		user_render_settings.output_dir,
		time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap()
	);

	image_buffer.save(&path::Path::new(&save_path_string)).unwrap();

	println!("Rendering completed in {} ms. Goodbye!\n", start_time.elapsed().as_millis());

}

// returns the home directory as specified in the $HOME environment variable
fn get_home_directory() -> String {
	match env::var("HOME") {
		Ok(p) => p,
		Err(_) => String::from("/"),
	}
}

// remaps x from [a0,a1] to [b0,b1] 
fn remap(x: f64, a: (f64, f64), b: (f64, f64)) -> f64 {
	b.0 + (x - a.0) * (b.1 - b.0) / (a.1 - a.0)
}
