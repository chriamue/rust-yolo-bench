use crate::{image_queue::ImageQueue, Model};
use std::sync::{Arc, Mutex, TryLockError};
use web_sys::ImageData;

pub struct Pipeline {
    model: Mutex<Model>,
    video_queue: Arc<ImageQueue>,
    processed_queue: Arc<ImageQueue>,
    processor: Mutex<Option<Arc<dyn crate::yolo::Processor>>>,
}

impl Pipeline {
    pub fn new(
        model: Model,
        video_queue: Arc<ImageQueue>,
        processed_queue: Arc<ImageQueue>,
    ) -> Self {
        Pipeline {
            model: Mutex::new(model),
            video_queue,
            processed_queue,
            processor: Mutex::new(None),
        }
    }

    fn to_dynamic_image(image_data: ImageData) -> image::DynamicImage {
        let img = image::ImageBuffer::from_raw(
            image_data.width(),
            image_data.height(),
            image_data.data().to_vec(),
        )
        .unwrap();
        image::DynamicImage::ImageRgba8(img)
    }

    fn from_dynamic_image(image: image::DynamicImage) -> ImageData {
        let img = image.to_rgba8();
        let width = img.width();
        let height = img.height();
        ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&img.into_raw()),
            width,
            height,
        )
        .unwrap()
    }

    pub fn process(&self) -> Result<(), Box<dyn std::error::Error + '_>> {
        match self.processor.try_lock() {
            Ok(mut processor_guard) => {
                if let Some(mut image_data) = self.video_queue.pop() {
                    if let Some(processor) = processor_guard.as_mut() {
                        let mut image = Box::new(Pipeline::to_dynamic_image(image_data));
                        processor.process(&mut image)?;
                        image_data = Pipeline::from_dynamic_image(*image);
                    }
                    self.processed_queue.push(image_data)?;
                }
            }
            Err(TryLockError::WouldBlock) => {
                log::warn!("Unable to acquire locks for processor.");
                return Ok(());
            }
            _ => {
                return Err("Failed to acquire necessary locks".into());
            }
        }

        Ok(())
    }

    pub fn set_model(&self, model: Model) {
        let mut model_guard = self.model.lock().unwrap();
        *model_guard = model;
        match *model_guard {
            #[cfg(feature = "candle")]
            Model::Candle => {
                let mut processor_guard = self.processor.lock().unwrap();
                *processor_guard = Some(Arc::new(crate::yolo::candle::Yolo::default()));
            }
            #[cfg(feature = "tract")]
            Model::Tract => {
                let mut processor_guard = self.processor.lock().unwrap();
                *processor_guard = Some(Arc::new(crate::yolo::tract::Yolo::default()));
            }
            Model::None => {
                let mut processor_guard = self.processor.lock().unwrap();
                *processor_guard = None;
            }
        }
    }
}
