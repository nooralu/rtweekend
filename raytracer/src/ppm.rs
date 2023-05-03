use std::sync::mpsc::{channel, Receiver, Sender};

use vec3::Color;

pub type Pixel = ((u32, u32), Color);

pub struct PPMGenerator {
    width: u32,
    height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    sender: Sender<Pixel>,
    receiver: Receiver<Pixel>,
    image: Vec<Vec<Color>>,
}

impl PPMGenerator {
    pub fn new_with(width: u32, aspect_ratio: f64, samples_per_pixel: u32, max_depth: u32) -> Self {
        let (sender, receiver) = channel();
        let height: u32 = (width as f64 / aspect_ratio) as u32;
        Self {
            width,
            height,
            samples_per_pixel,
            max_depth,
            sender,
            receiver,
            image: vec![vec![Default::default(); width as usize]; height as usize],
        }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn samples_per_pixel(&self) -> u32 {
        self.samples_per_pixel
    }

    pub fn max_depth(&self) -> u32 {
        self.max_depth
    }

    pub fn sender(&self) -> Sender<Pixel> {
        self.sender.clone()
    }

    pub fn save(&mut self) {
        let ppm_header = format!("P3\n{} {}\n255\n", self.width, self.height);
        let mut ppm_body = String::new();

        let height = self.height as usize;
        let width = self.width as usize;

        for j in (0..height).rev() {
            eprintln!("\rScanlines remaining: {j}");
            for _ in 0..width {
                let pixel = self.receiver.recv().unwrap();
                self.image[pixel.0 .1 as usize][pixel.0 .0 as usize] = pixel.1;
            }
        }
        for j in (0..height).rev() {
            for i in 0..width {
                ppm_body += &format_color(&self.image[j][i], self.samples_per_pixel);
            }
        }
        print!("{}{}", ppm_header, ppm_body);
        eprintln!("Done.");
    }
}

pub fn format_color(pixel_color: &Color, samples_per_pixel: u32) -> String {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    // Format the translated [0,255] value of each color component.
    format!(
        "{ir} {ig} {ib}\n",
        ir = (255.999 * r.clamp(0.0, 0.999)) as i32,
        ig = (255.999 * g.clamp(0.0, 0.999)) as i32,
        ib = (255.999 * b.clamp(0.0, 0.999)) as i32
    )
}
