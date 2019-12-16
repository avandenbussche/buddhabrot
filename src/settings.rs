use std::io::prelude::*;
use std::io;
use std::path;

pub struct ViewSettings {
	pub x_min: f64,
	pub x_max: f64,
	pub y_min: f64,
	pub y_max: f64
}

pub struct OutputSettings {
	pub width: u32,
	pub output_dir: String
}

impl OutputSettings {
	
	pub fn getHeightFromViewSettings(&self, view_settings: &ViewSettings) -> u32 {
		let resolution = self.width as f64 / (view_settings.x_min - view_settings.x_max).abs();
		(resolution.floor() * (view_settings.y_min - view_settings.y_max).abs()) as u32
	}

}

// prompts the user for their view settings
pub fn prompt_view_properties(defaults: ViewSettings) -> ViewSettings {

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
pub fn prompt_output_properties(defaults: OutputSettings) -> OutputSettings {

	println!("Please enter your settings for output. Press ENTER to accept the default value.");

	let width: u32;
	let output_dir: String;

	// ask for width
	loop {

		print!("WIDTH: ({}) ", defaults.width);
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

	println!("HEIGHT: Will be automatically calculated so as to preserve a 1:1 aspect ratio.");

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