use std::ops::Range;

use glam::DVec3 as Color;

pub fn convert_to_rgb(color:Color) {
    let r:f64 = linear_to_gamma(color.x);
    let g:f64 = linear_to_gamma(color.y);
    let b:f64 = linear_to_gamma(color.z);

    let intensity:Range<f64> = 0.000..0.999;
    let ir: u8 = (255.999 * r.clamp(intensity.start, intensity.end)) as u8;
    let ig: u8 = (255.999 * g.clamp(intensity.start, intensity.end)) as u8;
    let ib: u8 = (255.999 * b.clamp(intensity.start, intensity.end)) as u8;
    
    println!("{} {} {}", ir, ig, ib);
}

fn linear_to_gamma(linear_coponent:f64) -> f64 {
    if linear_coponent > 0.0 {
        return linear_coponent.sqrt();
    };
    return 0.0
}