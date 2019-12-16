pub mod settings;
pub mod complex;

extern crate png;

use std::env;


fn main() {

	println!("\nWelcome to the buddhabrot generator!\n");

	let default_view_settings = settings::ViewSettings {
		x_min: -2.0,
		x_max: 2.0,
		y_min: -1.2,
		y_max: 1.2
	};

	let user_view_settings = settings::prompt_view_properties(default_view_settings);

	let default_output_settings = settings::OutputSettings {
		width: 1920,
		output_dir: format!("{}/", get_home_directory())
	};

	let user_output_settings = settings::prompt_output_properties(default_output_settings);


	// mandelbrot set demo
	let num_pixels = user_output_settings.width * user_output_settings.get_height_from_view_settings(&user_view_settings);
	let resolution = user_output_settings.get_resolution_from_view_settings(&user_view_settings);
	let mut current_x = user_view_settings.x_min;
	let mut current_y: f64;

	let mut converged = false;
	let mut z: complex::Complex;
	let mut num_iterations_until_covergence: Vec<u32> = Vec::with_capacity(num_pixels as usize);

	while current_x <= user_view_settings.x_max {

		current_y = user_view_settings.y_min;

		while current_y <= user_view_settings.y_max {

			z = complex::Complex(0.0, 0.0);

			for i in 0..10000 {
				z.squared();
				z.add(complex::Complex(current_x, current_y));

				// if the modulus ever grows larger than 2, it will never converge
				if z.abs() > 2.0 {
					num_iterations_until_covergence.push(i);
					converged = true;
					break;
				}
			}

			if !converged {
				num_iterations_until_covergence.push(0);
			}

			current_y += resolution;
			converged = false;
		}

		current_x += resolution;
	}

	for i in num_iterations_until_covergence{
		print!("{} ", i);
	}

}

// returns the home directory as specified in the $HOME environment variable
fn get_home_directory() -> String {
	match env::var("HOME") {
		Ok(p) => p,
		Err(_) => String::from("/")
	}
}