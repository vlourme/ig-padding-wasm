extern crate alloc;
extern crate wasm_bindgen;

use std::cmp::max;

use image::{codecs::jpeg::JpegEncoder, DynamicImage, GenericImage, Rgba};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use lol_alloc::{FreeListAllocator, LockedAllocator};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> = LockedAllocator::new(FreeListAllocator::new());

#[derive(Deserialize)]
pub struct ImageConfig {
    pub borders: u32,
    pub color: String,
}

#[wasm_bindgen]
pub fn read_image(data: Vec<u8>, config: JsValue) -> Vec<u8> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let config: ImageConfig = serde_wasm_bindgen::from_value(config).unwrap();
    let image = image::load_from_memory(&data).unwrap();
    let image = image.to_rgba8();

    let (width, height) = image.dimensions();

    let (new_width, new_height) = (
        max(width, height) + config.borders,
        max(width, height) + config.borders,
    );

    let color = match config.color.as_str() {
        "black" => Rgba([0, 0, 0, 255]),
        _ => Rgba([255, 255, 255, 255]),
    };

    let mut new_image = DynamicImage::new_rgb8(new_width, new_height);

    // Set white
    for x in 0..new_width {
        for y in 0..new_height {
            new_image.put_pixel(x, y, color);
        }
    }

    // put base image at the center
    let x_offset = (new_width - width) / 2;
    let y_offset = (new_height - height) / 2;

    for x in 0..width {
        for y in 0..height {
            new_image.put_pixel(x + x_offset, y + y_offset, *image.get_pixel(x, y));
        }
    }

    let mut result = Vec::new();

    JpegEncoder::new(&mut result)
        .encode(
            &new_image.into_rgb8().into_vec(),
            new_width,
            new_height,
            image::ColorType::Rgb8,
        )
        .unwrap();

    result
}
