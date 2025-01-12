use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use crate::{
    shapes::world::World,
    utilities::{
        camera::{Camera, CameraSettings},
        image::Image,
    },
};

use serde::{Deserialize, Serialize};
use serde_yml;

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub render: RenderSettings,
    pub camera: CameraSettings,
}

impl Settings {
    pub fn new(render: RenderSettings, camera: CameraSettings) -> Self {
        Self { render, camera }
    }
}

#[derive(Deserialize, Serialize)]
pub struct RenderSettings {
    /// Sample rays cast per pixel
    pub msaa_samples: f64,
    /// Maximum number of times a ray is allowed to bounce
    pub max_depth: u64,
    /// Gamma correction applied after render
    pub gamma: f64,
    /// Initial time the camera shutter was opened
    pub shutter_open: f64,
    /// Time the shutter was closed
    pub shutter_close: f64,
}

impl RenderSettings {
    pub fn new(
        msaa_samples: f64,
        max_depth: u64,
        gamma: f64,
        shutter_open: f64,
        shutter_close: f64,
    ) -> Self {
        Self {
            msaa_samples,
            max_depth,
            gamma,
            shutter_open,
            shutter_close,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Scene {
    /// Render configuration
    pub settings: Settings,
    /// Image output size
    pub image: Image,
    /// Camera location
    #[serde(skip_serializing, skip_deserializing)]
    pub camera: Camera,
    /// Objects to render
    pub world: World,
}

impl Scene {
    pub fn new(settings: Settings, image: Image, camera: Camera, world: World) -> Self {
        Self {
            settings,
            image,
            camera,
            world,
        }
    }

    pub fn render(&self, filepath: &str, filename: &str) {
        self.image
            .save(filepath, filename, self.settings.render.gamma)
    }

    fn path(filepath: &str, filename: &str) -> PathBuf {
        Path::new(filepath).join(format!("{filename}.scene"))
    }

    pub fn save(&self, filepath: &str, filename: &str) {
        // Generate path
        let path = Scene::path(filepath, filename);

        // Create file at location
        let file = File::create(&path).unwrap();
        let buf_file = BufWriter::new(file);

        println!("Writing scene...");
        serde_yml::to_writer(buf_file, self).unwrap();
        println!("Wrote scene to {:0}", path.as_os_str().to_str().unwrap());
    }

    pub fn load(filepath: &str, filename: &str) -> Self {
        // Generate path
        let path = Scene::path(filepath, filename);
        let file = File::open(&path).unwrap();

        // Read and parse scene file
        let buf_file = BufReader::new(file);
        let mut scene: Self = serde_yml::from_reader(buf_file).unwrap();

        // Fill image buffer
        scene.image.buffer = Image::generate_buffer(scene.image.width, scene.image.height);

        // Update camera aspect ratio for image
        scene.settings.camera.aspect_ratio = scene.image.aspect_ratio();

        // Build camera for scene
        scene.camera = Camera::new(&scene.settings.camera);
        scene
    }
}
