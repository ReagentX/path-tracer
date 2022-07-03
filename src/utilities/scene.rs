use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use crate::{
    shapes::world::World,
    utilities::{camera::Camera, image::Image},
};

use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Deserialize, Serialize)]
pub struct Scene {
    pub camera: Camera,
    pub image: Image,
    pub world: World,
}

impl Scene {
    pub fn new(camera: Camera, image: Image, world: World) -> Self {
        Self {
            camera,
            image,
            world,
        }
    }

    pub fn render(&self, filepath: &str, filename: &str, gamma: f64) {
        self.image.save(filepath, filename, gamma)
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
        serde_yaml::to_writer(buf_file, self).unwrap();
        println!("Wrote scene to {:0}", path.as_os_str().to_str().unwrap());
    }

    pub fn load(filepath: &str, filename: &str) -> Self {
        // Generate path
        let path = Scene::path(filepath, filename);
        let file = File::open(&path).unwrap();
        let buf_file = BufReader::new(file);

        let mut scene: Self = serde_yaml::from_reader(buf_file).unwrap();
        scene.image.buffer = Image::generate_buffer(scene.image.width, scene.image.height);
        scene
    }
}
