use image::RgbImage;
use std::env;
use std::f64::consts::PI;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use lib::examples::example_suzanne as scene;
use lib::optics::{Camera, RenderOutputConfig};
use lib::utils::Vec3;

use RenderMode::{Dev, Latest};

fn render(render_mode: RenderMode, timestep: f64) -> RgbImage {
    let (world, mut viewport_config, lens_config) = scene();

    let render_output_config = match render_mode {
        Dev => RenderOutputConfig {
            aspect_ratio: 16.0 / 9.0,
            image_width: 1920,
            samples_per_pixel: 32,
            max_depth: 32,
        },
        Latest => RenderOutputConfig {
            aspect_ratio: 16.0 / 9.0,
            image_width: 3840,
            samples_per_pixel: 32,
            max_depth: 32,
        },
    };

    let delta = 2. * PI * timestep;
    viewport_config.look_from += Vec3::new(
        5. * (2. * delta).cos(),
        2.0 * (delta - 0.5).sin(),
        5. * -(2. * delta).sin(),
    );

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

    let _output_path = match render_mode {
        Dev => Path::new("images/output.png"),
        Latest => Path::new("latest.png"),
    };

    let num_timesteps = 20;
    for timestep in 0..num_timesteps {
        println!("Rendering timestep {}", timestep);
        let (tx, rx) = mpsc::channel();
        let n_threads = 8;

        for _ in 0..n_threads {
            let tx = tx.clone();
            let render_mode = render_mode.clone();
            thread::spawn(move || {
                let image = render(render_mode, timestep as f64 / num_timesteps as f64);
                tx.send(image).unwrap();
            });
        }

        let images: Vec<RgbImage> = rx.iter().take(n_threads).collect();
        let mut output_image: RgbImage = RgbImage::new(images[0].width(), images[0].height());

        Camera::aggregate(&mut output_image, &images);
        output_image
            .save(Path::new(&format!("images/output/{}.png", timestep)))
            .unwrap();
    }
}
