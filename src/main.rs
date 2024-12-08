mod blend;
mod io;
mod operations;

use io::*;
use operations::{
    AdditionBlend, AverageBlend, DarkenBlend, LightenBlend, MultiplyBlend, ScreenBlend,
    SubtractionBlend,
};

fn main() {
    let source_data = read_pixel_data(
        "Hades_Times_a_matter_of_life_and_death_abstract_surreal_glitch_111bf1e7-0d80-40ff-917b-9c5a919db759.png".to_string(),
        "Hades_Times_a_matter_of_life_and_death_abstract_surreal_glitch_c47a5eeb-6627-4a45-935f-6864252d33c3.png".to_string()
    );

    let output_buffer = source_data.blend_images(AdditionBlend);
    output_buffer.save("addition.jpg").unwrap();

    let output_buffer = source_data.blend_images(AverageBlend);
    output_buffer.save("average.jpg").unwrap();

    let output_buffer = source_data.blend_images(DarkenBlend);
    output_buffer.save("darken.jpg").unwrap();

    let output_buffer = source_data.blend_images(LightenBlend);
    output_buffer.save("lighten.jpg").unwrap();

    let output_buffer = source_data.blend_images(MultiplyBlend);
    output_buffer.save("multiply.jpg").unwrap();

    let output_buffer = source_data.blend_images(ScreenBlend);
    output_buffer.save("screen.jpg").unwrap();

    let output_buffer = source_data.blend_images(SubtractionBlend);
    output_buffer.save("subtraction.jpg").unwrap();
}
