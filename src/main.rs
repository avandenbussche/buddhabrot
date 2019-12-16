extern crate png;

use std::io::prelude::*;
use std::io;
use std::path;
use std::env;

struct ViewSettings {
	x_min: f64,
	x_max: f64,
	y_min: f64,
	y_max: f64
}

struct OutputSettings {
	width: u32,
	output_dir: String
}

fn main() {

	println!("Hello, world!");

	let default_view_settings = ViewSettings {
		x_min: -2.0,
		x_max: 2.0,
		y_min: -1.2,
		y_max: 1.2
	};

	let user_view_settings = prompt_view_properties(default_view_settings);

	let default_output_settings = OutputSettings {
		width: 1920,
		output_dir: format!("{}/", get_home_directory())
	};

	let user_output_settings = prompt_output_properties(default_output_settings);

}

// prompts the user for their view settings
fn prompt_view_properties(defaults: ViewSettings) -> ViewSettings {

	println!("Please enter your settings for the view. Press ENTER to accept the default value.");

	let x_min: f64;
	let x_max: f64;
	let y_min: f64;
	let y_max: f64;

	// ask for x_min
	loop {

		print!("XMIN: ({}) ", defaults.x_min);
		io::stdout().flush().ok().expect("Could not flush output."); // Forces output

		let mut input_x_min = String::new();
		io::stdin().read_line(&mut input_x_min)
			.expect("Failed to read XMIN.");
		
		input_x_min = input_x_min.trim().to_string();

		// if the user simply hits ENTER, assume default
		if input_x_min.len() == 0 {
			x_min = defaults.x_min;
		} else {
			x_min = match input_x_min.parse() {
				Ok(num) => num,
				Err(_) => continue
			};
		}

		break
		
	}

	// ask for x_max
	loop {

		print!("XMAX: ({}) ", defaults.x_max);
		io::stdout().flush().ok().expect("Could not flush output."); // Forces output

		let mut input_x_max = String::new();
		io::stdin().read_line(&mut input_x_max)
			.expect("Failed to read XMAX.");
		
		input_x_max = input_x_max.trim().to_string();

		// if the user simply hits ENTER, assume default
		if input_x_max.len() == 0 {
			x_max = defaults.x_max;
		} else {
			x_max = match input_x_max.parse() {
				Ok(num) => num,
				Err(_) => continue
			};
		}
		
		break

	}

	// ask for y_min
	loop {

		print!("YMIN: ({}) ", defaults.y_min);
		io::stdout().flush().ok().expect("Could not flush output."); // Forces output

		let mut input_y_min = String::new();
		io::stdin().read_line(&mut input_y_min)
			.expect("Failed to read YMIN.");
		
		input_y_min = input_y_min.trim().to_string();

		// if the user simply hits ENTER, assume default
		if input_y_min.len() == 0 {
			y_min = defaults.y_min;
		} else {
			y_min = match input_y_min.parse() {
				Ok(num) => num,
				Err(_) => continue
			};
		}

		break
		
	}

	// ask for y_max
	loop {

		print!("YMAX: ({}) ", defaults.y_max);
		io::stdout().flush().ok().expect("Could not flush output."); // Forces output

		let mut input_y_max = String::new();
		io::stdin().read_line(&mut input_y_max)
			.expect("Failed to read YMAX.");
		
		input_y_max = input_y_max.trim().to_string();

		// if the user simply hits ENTER, assume default
		if input_y_max.len() == 0 {
			y_max = defaults.y_max;
		} else {
			y_max = match input_y_max.parse() {
				Ok(num) => num,
				Err(_) => continue
			};
		}
		
		break

	}

	ViewSettings {
		x_min,
		x_max,
		y_min,
		y_max
	}

}


// prompts the user for their output settings
fn prompt_output_properties(defaults: OutputSettings) -> OutputSettings {

	println!("Please enter your settings for output. Press ENTER to accept the default value.");

	let width: u32;
	let output_dir: String;

	// ask for width
	loop {

		print!("XMIN: ({}) ", defaults.width);
		io::stdout().flush().ok().expect("Could not flush output."); // Forces output

		let mut input_width = String::new();
		io::stdin().read_line(&mut input_width)
			.expect("Failed to read WIDTH.");
		
		input_width = input_width.trim().to_string();

		// if the user simply hits ENTER, assume default
		if input_width.len() == 0 {
			width = defaults.width;
		} else {
			width = match input_width.parse() {
				Ok(num) => num,
				Err(_) => continue
			};
		}

		break
		
	}

	// ask for output directory
	loop {

		print!("OUTPUT DIRECTORY: ({}) ", defaults.output_dir);
		io::stdout().flush().ok().expect("Could not flush output."); // Forces output

		let mut input_output_dir = String::new();
		io::stdin().read_line(&mut input_output_dir)
			.expect("Failed to read OUTPUT DIRECTORY.");
		
		input_output_dir = input_output_dir.trim().to_string();

		// if the user simply hits ENTER, assume default
		if input_output_dir.len() == 0 {
			output_dir = defaults.output_dir;
		} else {
			output_dir = match path::Path::new(&input_output_dir[..]).exists() {
				true => input_output_dir,
				false => { println!("Specified directory does not exist!"); continue }
			};
		}

		break
		
	}

	OutputSettings {
		width,
		output_dir
	}

}

fn get_home_directory() -> String {
	match env::var("HOME") {
		Ok(p) => p,
		Err(_) => String::from("/")
	}
}