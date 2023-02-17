use image::{ GenericImageView, ImageBuffer, Rgba};
use indicatif::ProgressBar;
use std::path::PathBuf;
use structopt::StructOpt;

mod pixel_calculations;
use pixel_calculations::{vari, red_channel, green_channel, blue_channel, green_minus_blue};

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    #[structopt(parse(from_os_str))]
    output: PathBuf,

    #[structopt(short, long, possible_values=&["red", "green", "blue", "green-minus-blue", "vari"])]
    calculation: String,
}

fn lerp(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 * (1.0 - t) + b as f32 * t) as u8
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let calc_fn: fn(&Rgba<u8>) -> f32 = match args.calculation.as_str() {
        "vari" => vari,
        "red" => red_channel,
        "green" => green_channel,
        "blue" => blue_channel,
        "green-minus-blue" => green_minus_blue,
        _ => unreachable!(),
    };

    let img = image::open(args.input)?;

    let count = img.width() as u64 * img.height() as u64;
    let gradient_pb = ProgressBar::new(count);
    let pixel_calc_pb = ProgressBar::new(count);

    let mut out_img = ImageBuffer::new(img.width(), img.height());

    println!("Calculating Gradient Values...");
    // Find the minimum and maximum calculated values
    let (min_val, max_val) = img.pixels().fold((std::f32::INFINITY, std::f32::NEG_INFINITY), |(min, max), (_, _, pixel)| {
        let val = calc_fn(&pixel);
        gradient_pb.inc(1);
        (min.min(val), max.max(val))
    });
    gradient_pb.finish_with_message("Done!");

    println!("Calculating Pixel Values...");
    for (x, y, pixel) in img.pixels() {
        let val = calc_fn(&pixel);
        // Interpolate between two red and green based on the calculated value
        // TODO: Handle middle case properly for VARI calculation
        let t = (val - min_val) / (max_val - min_val);
        let colour = [lerp(255, 0, t), lerp(0, 255, t), 0, 1];
        out_img.put_pixel(x, y, Rgba(colour));
        pixel_calc_pb.inc(1);
    }

    pixel_calc_pb.finish_with_message("Done!");

    println!("Writing to Output Image Path...");
    out_img.save(args.output)?;

    println!("Done!");

    Ok(())
}
