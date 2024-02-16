use image::RgbImage;
use std::env;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use lib::optics::{Camera, RenderOutputConfig};
use lib::worlds::example_motion_blur;

use RenderMode::{Dev, Latest};

fn render(render_mode: RenderMode) -> RgbImage {
    let (world, viewport_config, lens_config) = example_motion_blur();

    let render_output_config = match render_mode {
        Dev => RenderOutputConfig {
            aspect_ratio: 16.0 / 9.0,
            image_width: 1080,
            samples_per_pixel: 25,
            max_depth: 30,
        },
        Latest => RenderOutputConfig {
            aspect_ratio: 16.0 / 9.0,
            image_width: 3840,
            samples_per_pixel: 50,
            max_depth: 60,
        },
    };

    let mut cam = Camera::new(render_output_config, viewport_config, lens_config);
    cam.render(&world)
}

#[derive(Clone)]
enum RenderMode {
    Dev,
    Latest,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let render_mode = if args.len() > 1 {
        match args[1].as_str() {
            "dev" => Dev,
            "latest" => Latest,
            _ => panic!("Invalid argument"),
        }
    } else {
        Dev
    };

    let output_path = match render_mode {
        Dev => Path::new("images/output.png"),
        Latest => Path::new("latest.png"),
    };

    let (tx, rx) = mpsc::channel();
    let n_threads = 8;

    for _ in 0..n_threads {
        let tx = tx.clone();
        let render_mode = render_mode.clone();
        thread::spawn(move || {
            let image = render(render_mode);
            tx.send(image).unwrap();
        });
    }

    let images: Vec<RgbImage> = rx.iter().take(n_threads).collect();
    let mut output_image: RgbImage = RgbImage::new(images[0].width(), images[0].height());

    Camera::aggregate(&mut output_image, &images);
    output_image.save(output_path).unwrap();
}
