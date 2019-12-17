use super::settings;

use std::path;
use std::time;

pub fn render_from_iterations(iteration_vector: &Vec<u32>, render_settings: &settings::RenderSettings) {

    let mut image_buffer: image::ImageBuffer<image::Rgb<u8>,_> = image::ImageBuffer::new(render_settings.output_width, render_settings.get_height());

    let mut i = 0;
    for current_pixel_y in 0..render_settings.get_height() { 
        for current_pixel_x in 0..render_settings.output_width {

            let g: u8;
            if iteration_vector[i] > 0 {
                g = remap(iteration_vector[i] as f64, (0.0, render_settings.max_iterations as f64), (0.0, 255.0)).round() as u8;
            } else {
                g = 255;
            }

            let this_pixel = image_buffer.get_pixel_mut(current_pixel_x, current_pixel_y);
            let image::Rgb(_data) = *this_pixel;
            *this_pixel = image::Rgb([g, g, g]);

            i += 1;

        }
    }

    // render output
    let save_path_string = format!(
        "{}mandelbrot_{:?}.png",
        render_settings.output_dir,
        time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs()
    );

    image_buffer.save(&path::Path::new(&save_path_string)).unwrap();

}

// remaps x from [a0,a1] to [b0,b1] 
fn remap(x: f64, a: (f64, f64), b: (f64, f64)) -> f64 {
	b.0 + (x - a.0) * (b.1 - b.0) / (a.1 - a.0)
}