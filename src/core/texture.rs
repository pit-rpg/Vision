extern crate image;
extern crate uuid;

use self::image::{ColorType, GenericImageView};
use self::uuid::Uuid;
use std::path::Path;
use std::sync::{Arc,Mutex, LockResult, MutexGuard};


#[allow(dead_code)]
#[derive(Debug)]
pub enum Wrapping {
	Repeat,
	MirroredRepeat,
	ClampToEdge,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Filtering {
	NEAREST,
	LINEAR,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TextureColorType {
	None,
	Gray(u8),
	RGB(u8),
	RGBA(u8),
}


#[allow(dead_code)]
#[derive(Debug)]
pub struct Texture2D {
	pub path: Option<String>,
	pub uuid: Uuid,
	pub wrapping_x: Wrapping,
	pub wrapping_y: Wrapping,
	pub filtering: Filtering,
	texture_data: Option<TextureData>,
}


#[derive(Debug)]
pub struct TextureData {
	pub color_type: TextureColorType,
	pub width: u32,
	pub height: u32,
	pub data: Vec<u8>, // TODO optional data for memory save
}

impl Texture2D {

	pub fn new (path: &str) -> Self {
		Self {
			path: Some(path.to_string()),
			uuid: Uuid::new_v4(),
			wrapping_x: Wrapping::Repeat,
			wrapping_y: Wrapping::Repeat,
			filtering: Filtering::NEAREST,
			texture_data: None,
		}
	}

	pub fn load (&self) -> Result<&TextureData, ()> {
		match (&self.path, &self.texture_data) {
			(None, Some(td)) | (Some(_), Some(td)) => {
				Ok(td)
			}
			(Some(path), None) => {
				let img =  match image::open(&Path::new(path)){
					Err(_) => {return Err(());}
					Ok(im) => im.flipv()
				};

				let color_type = match img.color() {
					ColorType::Gray(d) => TextureColorType::Gray(d),
					ColorType::RGB(d) =>  TextureColorType::RGB(d),
					ColorType::RGBA(d) => TextureColorType::RGBA(d),
					_ =>{ return Err(()) }
				};

				let data = img.raw_pixels();
				let (width, height) = img.dimensions();

				self.texture_data = Some(TextureData {
					data,
					width,
					height,
					color_type,
				});

				Ok(&self.texture_data.unwrap())
			}
			(None,None) => Err(())
		}
	}

	pub fn set_texture_data(&self, data: Option<TextureData>) {
		self.texture_data = data;
	}

	pub fn get_texture_data_ref (&self) -> Option<&TextureData> {
		self.texture_data.as_ref()
	}

	pub fn get_texture_data_ref_mut (&mut self) -> Option<&mut TextureData> {
		self.texture_data.as_mut()
	}
}


#[derive(Debug, Clone)]
pub struct SharedTexture2D (Arc<Mutex<Texture2D>>);


impl SharedTexture2D {
	pub fn new(texture: Texture2D) -> Self {
		SharedTexture2D(Arc::new(Mutex::new(texture)))
	}

	pub fn new_from_path(path: &str) -> Self {
		SharedTexture2D(Arc::new(Mutex::new(Texture2D::new(path))))
	}

	pub fn lock(&mut self) -> LockResult<MutexGuard<Texture2D>> {
		self.0.lock()
	}
}
