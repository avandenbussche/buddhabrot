use super::settings;
use super::complex;

extern crate rayon;

use rayon::prelude::*;

pub fn mandelbrot(render_settings: &settings::RenderSettings) -> Vec<u32> {

    let par_num_iterations_until_covergence_iter = (0..render_settings.get_num_pixels()).into_par_iter().map(|pixel_i| {

		let current_pixel_x = pixel_i % render_settings.output_width;
		let current_pixel_y = pixel_i / render_settings.output_width;

		let resolution_x = render_settings.get_resolution_x();
		let resolution_y = render_settings.get_resolution_y();

		let current_x = render_settings.x_min + current_pixel_x as f64 * resolution_x;
		let current_y = render_settings.y_min + current_pixel_y as f64 * resolution_y;

		let mut z = complex::Complex(0.0, 0.0);

		for i in 1..render_settings.max_iterations {

			z.squared();
			z.add(complex::Complex(current_x, current_y));

			// if the modulus ever grows larger than 2, it will never converge
			if z.norm_squared() > 4.0 {
                // point is not in mandelbrot set
				return i;
			}
		}

        // point is in mandelbrot set
		return 0;

	});

    par_num_iterations_until_covergence_iter.collect()
    
}