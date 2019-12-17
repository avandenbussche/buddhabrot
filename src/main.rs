pub mod complex;
pub mod settings;

extern crate png;

use std::env;
use std::fs;
use std::io;
use std::path;
use std::time;

fn main() {
	println!("\nWelcome to the buddhabrot generator!\n");

	let default_render_settings = settings::RenderSettings {
		x_min: -2.0,
		x_max: 0.5,
		y_min: -1.25,
		y_max: 1.25,
		max_iterations: 80,
		output_width: 400,
		output_dir: format!("{}/", get_home_directory()),
	};

	let user_render_settings = settings::prompt_render_settings(&default_render_settings);

	// mandelbrot set demo
	let num_pixels = user_render_settings.output_width * user_render_settings.get_height();
	let resolution_x = user_render_settings.get_resolution_x();
	let resolution_y = user_render_settings.get_resolution_y();
	let mut current_x: f64;
	let mut current_y = user_render_settings.y_max;
	let mut current_pixel_x: u32; // we also will keep track of pixels so as to avoid issues caused by floating point errors
	let mut current_pixel_y = 0;

	let mut converged = false;
	let mut z: complex::Complex;
	let mut num_iterations_until_covergence: Vec<u32> = Vec::with_capacity(num_pixels as usize);
	let mut most_iterations_required: u32 = 0;

	// start in top left corner and move first across cols, then down rows
	while current_pixel_y < user_render_settings.get_height() {
		current_pixel_x = 0;
		current_x = user_render_settings.x_min;

		while current_pixel_x < user_render_settings.output_width {
			z = complex::Complex(0.0, 0.0);

			for i in 1..user_render_settings.max_iterations {
				z.squared();
				z.add(complex::Complex(current_x, current_y));

				// if the modulus ever grows larger than 2, it will never converge
				if z.abs_squared() > 4.0 {
					
					if i > most_iterations_required {
						most_iterations_required = i;
					}

					num_iterations_until_covergence.push(i);
					converged = true;
					break;

				}
			}

			if !converged {
				num_iterations_until_covergence.push(0);
			}

			current_x += resolution_x;
			current_pixel_x += 1;
			converged = false;
		}

		current_y -= resolution_y;
		current_pixel_y += 1;
	}
	
	// convert num iterations to colors
	let mut pixel_colors: Vec<u8> = Vec::with_capacity(num_pixels as usize);

	for i in num_iterations_until_covergence {
		if i > 0 {
			pixel_colors.push( remap(i as f64, (0.0, most_iterations_required as f64), (0.0, 255.0)).round() as u8 );
		} else {
			pixel_colors.push(255);
		}
	}

	// render output
	let save_path_string = format!(
		"{}buddhabrot_{:?}.png",
		user_render_settings.output_dir,
		time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap()
	);
	let save_path = path::Path::new(&save_path_string);
	let save_file = fs::File::create(save_path).unwrap();
	let ref mut w = io::BufWriter::new(save_file);

	let mut encoder = png::Encoder::new(
		w,
		user_render_settings.output_width,
		user_render_settings.get_height(),
	);
	encoder.set_color(png::ColorType::Grayscale);
	encoder.set_depth(png::BitDepth::Eight);
	let mut writer = encoder.write_header().unwrap();

	// save the image
	writer.write_image_data(&pixel_colors).unwrap();

	println!("");
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
