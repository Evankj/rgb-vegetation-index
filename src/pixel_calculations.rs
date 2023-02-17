pub fn red_channel(pixel: &image::Rgba<u8>) -> f32 {
    pixel[0] as f32
}

pub fn green_channel(pixel: &image::Rgba<u8>) -> f32 {
    pixel[1] as f32
}

pub fn blue_channel(pixel: &image::Rgba<u8>) -> f32 {
    pixel[2] as f32
}

pub fn vari(pixel: &image::Rgba<u8>) -> f32 {
    let r = pixel[0] as f32;
    let g = pixel[1] as f32;
    let b = pixel[2] as f32;
    let vari = (g - r) / (g + r - b);
    vari
}

pub fn green_minus_blue(pixel: &image::Rgba<u8>) -> f32 {
    let g = pixel[1] as f32;
    let b = pixel[2] as f32;
    let green_minus_blue = g - b;
    green_minus_blue
}
