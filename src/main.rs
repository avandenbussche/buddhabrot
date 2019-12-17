pub mod complex;
pub mod settings;
pub mod render;
pub mod fractals;

extern crate image;

use std::env;
use std::time;

fn main() {
	println!("\nWelcome to the Mandelbrot set renderer!\n");

	let default_render_settings = settings::RenderSettings {
		x_min: -2.0,
		x_max: 0.5,
		y_min: -1.25,
		y_max: 1.25,
		max_iterations: 255,
		output_width: 1920,
		output_dir: format!("{}/", get_home_directory()),
	};

	let user_render_settings = settings::prompt_render_settings(&default_render_settings);

	// for timing
	let start_time = time::Instant::now();

	// mandelbrot set demo
	let iterations = fractals::mandelbrot(&user_render_settings);
	
	// convert num iterations to colors
	render::render_from_iterations(&iterations, &user_render_settings);

	println!("Rendering completed in {} ms. Enjoy!\n", start_time.elapsed().as_millis());

}

// returns the home directory as specified in the $HOME environment variable
fn get_home_directory() -> String {
	match env::var("HOME") {
		Ok(p) => p,
		Err(_) => String::from("/"),
	}
}