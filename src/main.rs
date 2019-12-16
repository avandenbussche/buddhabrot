pub mod settings;

extern crate png;

use std::env;


fn main() {

	println!("Hello, world!");

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

}
fn get_home_directory() -> String {
	match env::var("HOME") {
		Ok(p) => p,
		Err(_) => String::from("/")
	}
}