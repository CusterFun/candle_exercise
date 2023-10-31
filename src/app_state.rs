use crate::pose::model::{Multiples, YoloV8Pose};
use candle_core::DType;
use candle_nn::VarBuilder;
use lazy_static::lazy_static;
use opencv::videoio::VideoCapture;
use std::sync::{Arc, Mutex, RwLock};

pub type SharedState = Arc<RwLock<AppState>>;
#[derive(Default)]
pub struct AppState {
    pub camera: Option<Arc<Mutex<VideoCapture>>>,
}

pub struct Singleton {
    pub pose_model: Option<YoloV8Pose>,
}

lazy_static! {
    pub static ref SINGLETON_INSTANCE: Mutex<Singleton> = Mutex::new(Singleton::default());
}

impl Singleton {
    fn default() -> Self {
        let mut singleton = Self { pose_model: None };
        // Create the model and load the weights from the file.
        let multiples = Multiples::s();
        let model = std::path::PathBuf::from("yolov8s-pose.safetensors");
        let device = match crate::pose::utils::device(false) {
            Ok(device) => device,
            Err(e) => {
                tracing::error!("加载模型失败: {}", e);
                return singleton;
            }
        };
        let vb = match unsafe { VarBuilder::from_mmaped_safetensors(&[model], DType::F32, &device) }
        {
            Ok(vb) => vb,
            Err(e) => {
                tracing::error!("加载模型失败: {}", e);
                return singleton;
            }
        };
        let pose_model = match YoloV8Pose::load(vb, multiples, /* num_classes=*/ 1, (17, 3)) {
            Ok(model) => model,
            Err(e) => {
                tracing::error!("加载模型失败: {}", e);
                return singleton;
            }
        };
        tracing::debug!("成功加载模型");
        singleton.pose_model = Some(pose_model);
        singleton
    }
}
