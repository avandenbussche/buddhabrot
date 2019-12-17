# Mandelbrot

A very basic Mandelbrot set renderer with parallelization written in Rust. This project was started as a way to get a feel for simple Rust syntax.

## Instructions

Clone this repo and use `cargo run` to build and run the crate. You will be prompted for your render settings, then rendering will begin.

## Sample Output

### Classic View

Sample rendering with the default settings.

![Classic Mandelbrot Set Rendering](/sample_images/main_mandelbrot_readme.png)

### Enlarged Cool Feature

Sample rendering with the following settings:

```
Please enter your desired render settings. Press ENTER to accept the default value.
XMIN: (-2) -0.54
XMAX: (0.5) -0.52
YMIN: (-1.25) 0.66
YMAX: (1.25) 0.68
MAX ITERATIONS: (255) 
OUTPUT WIDTH: (1920) 10000
OUTPUT HEIGHT will be automatically calculated.
OUTPUT DIRECTORY: (/Users/avandenbussche/) 
```

![Enlarged Cool Feature on Edge of Mandelbrot Set](/sample_images/cool_mandelbrot_readme.png)
