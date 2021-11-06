use image;
use std::format;
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;

// Function to map the number of iterations i to a grey value between 0 (black)
// and 255 (white).
fn get_color(i: u32, max_iterations: u32) -> image::Rgb<u8> {
    if i > max_iterations {
        return image::Rgb([255, 255, 255]);
    }
    if max_iterations == 255 {
        let idx = i as u8;
        return image::Rgb([idx, idx, idx]);
    }
    let idx = (((i as f32) / (max_iterations as f32)) * 255.0).round() as u8;
    return image::Rgb([idx, idx, idx]);
}

// Function to run a Mandelbrot rendering algorithm and measure its execution
// time.
// Arguments:
//   name: name of the algorithm, it's used to print its name and save the output.
//   w: width of the output image, in pixels.
//   h: height of the output image, in pixels.
//   save_image: if true, save the output of the algorithm to
//               /tmp/mandelbrot_{name}.png
//   algo: actual rendering algorithm that should take as inputs the width and
//         height of the output image and returns an image::RgbImage
fn runalgo(name: &str, w: u32, h: u32, max_itr: u32, save_image: bool, algo: fn(u32, u32, u32) -> image::RgbImage) {
    print!("Executing {}... ", name);
    io::stdout().flush().unwrap();
    let now = Instant::now();
    let img = algo(w, h, max_itr);
    let elapsed = now.elapsed().as_millis() as f32 / 1000.0;
    if save_image {
        let fname = format!("mandelbrot_{}.png", name);
        img.save_with_format(Path::new("/tmp").join(fname), image::ImageFormat::Png)
            .unwrap();
    }
    println!("{}s", elapsed);
}

// Render the Mandelbrot set. w and h are respectively the widht and height of
// the output image. Refer to
// https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set for
// a detailed explanation of this algorithm.
fn naive(w: u32, h: u32, max_itr: u32) -> image::RgbImage {
    let mut img = image::RgbImage::new(w, h);
    for c in 0..w {
        let x0 = ((c as f32) / (w as f32)) * 3.5 - 2.5;
        for r in 0..h {
            let y0 = ((r as f32) / (h as f32)) * 2.0 - 1.0;
            let mut x = 0.0;
            let mut y = 0.0;
            let mut iteration: u32 = 0;
            while x * x + y * y <= 4.0 && iteration < max_itr {
                let xtemp = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = xtemp;
                iteration = iteration + 1;
            }
            let rgb = get_color(iteration, max_itr);
            img.put_pixel(c, r, rgb);
        }
    }
    img
}

fn main() {
    let width = 14000;
    let height = 8000;
	let max_iterations = 500;
    let save_image = true;
    runalgo("naive", width, height, max_iterations, save_image, naive);
}
