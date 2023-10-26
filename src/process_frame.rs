use crate::app_state::SINGLETON_INSTANCE;
use crate::pose::model::YoloV8Pose;
use candle_core::{DType, Device, Module, Tensor};
use image::{DynamicImage, ImageBuffer, Rgb};
use opencv::core::{Mat, MatTraitConst, MatTraitConstManual};
use std::time::SystemTime;

use crate::pose::yolov8::Task;

pub fn process_frame_with_candle(
    mat: &Mat,
    confidence_threshold: Option<f32>,
    nms_threshold: Option<f32>,
    legend_size: Option<u32>,
) -> anyhow::Result<DynamicImage> {
    let instance = SINGLETON_INSTANCE.lock().unwrap();
    let model: &YoloV8Pose = match instance.pose_model.as_ref() {
        Some(model) => model,
        None => {
            return Err(anyhow::anyhow!("加载模型失败"));
        }
    };
    // 将opencv的Mat转换为image::DynamicImage
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(
        mat.cols() as u32,
        mat.rows() as u32,
        mat.data_typed::<u8>().unwrap().to_vec(),
    )
    .unwrap();
    let original_image = DynamicImage::ImageRgb8(img);
    tracing::debug!("start processing frame...");

    let (width, height) = {
        let w = original_image.width() as usize;
        let h = original_image.height() as usize;
        if w < h {
            let w = w * 640 / h;
            // Sizes have to be divisible by 32.
            (w / 32 * 32, 640)
        } else {
            let h = h * 640 / w;
            (640, h / 32 * 32)
        }
    };
    let image_t = {
        let img = original_image.resize_exact(
            width as u32,
            height as u32,
            image::imageops::FilterType::CatmullRom,
        );
        let data = img.to_rgb8().into_raw();
        Tensor::from_vec(
            data,
            (img.height() as usize, img.width() as usize, 3),
            &Device::Cpu,
        )?
        .permute((2, 0, 1))?
    };
    let image_t = (image_t.unsqueeze(0)?.to_dtype(DType::F32)? * (1. / 255.))?;
    let predictions = model.forward(&image_t)?.squeeze(0)?;
    tracing::debug!("generated predictions {predictions:?}");
    let image_t = YoloV8Pose::report(
        &predictions,
        original_image,
        width,
        height,
        confidence_threshold.unwrap_or(0.25),
        nms_threshold.unwrap_or(0.45),
        legend_size.unwrap_or(14),
    )?;
    let image_name = std::path::PathBuf::from(format!("{:?}.jpg", SystemTime::now()));
    tracing::debug!("writing {image_name:?}");
    image_t.save(image_name)?;
    Ok(image_t)
}
