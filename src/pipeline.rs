use std::sync::{Arc, Mutex, TryLockError};

use web_sys::ImageData;

use crate::{image_queue::ImageQueue, Model};

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
        ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&mut img.into_raw()),
            image.width(),
            image.height(),
        )
        .unwrap()
    }

    pub fn process(&self) -> Result<(), Box<dyn std::error::Error + '_>> {
        match (self.model.try_lock(), self.processor.try_lock()) {
            (Ok(model_guard), Ok(mut processor_guard)) => match *model_guard {
                #[cfg(feature = "candle")]
                Model::Candle => {
                    if processor_guard.is_none() {
                        *processor_guard = Some(Arc::new(crate::yolo::candle::Yolo::default()));
                    }

                    if let Some(image_data) = self.video_queue.pop() {
                        let processor = processor_guard.as_ref().unwrap();
                        let mut img = Box::new(Pipeline::to_dynamic_image(image_data));
                        processor.process(&mut img)?;
                        let image_data = Pipeline::from_dynamic_image(*img);
                        self.processed_queue.push(image_data)?;
                    }
                }
                #[cfg(feature = "tract")]
                Model::Tract => {
                    if processor_guard.is_none() {
                        *processor_guard = Some(Arc::new(crate::yolo::tract::Yolo::default()));
                    }

                    if let Some(image_data) = self.video_queue.pop() {
                        let processor = processor_guard.as_ref().unwrap();
                        let mut img = Box::new(Pipeline::to_dynamic_image(image_data));
                        processor.process(&mut img)?;
                        let image_data = Pipeline::from_dynamic_image(*img);
                        self.processed_queue.push(image_data)?;
                    }
                }
                Model::None => {
                    if let Some(image_data) = self.video_queue.pop() {
                        self.processed_queue.push(image_data)?;
                    }
                }
            },
            (Err(TryLockError::WouldBlock), _) | (_, Err(TryLockError::WouldBlock)) => {
                log::warn!("Unable to acquire locks for model or processor.");
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
