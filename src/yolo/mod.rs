use image::DynamicImage;

#[cfg(feature = "tract")]
pub mod tract;

#[cfg(feature = "candle")]
pub mod candle;

pub trait Processor {
    fn process(&self, image: &mut Box<DynamicImage>) -> Result<(), Box<dyn std::error::Error>>;
}
