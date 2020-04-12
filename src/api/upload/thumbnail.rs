use crate::api::Conf;
use image::{self, ImageFormat};
use rocket::{http::ContentType, response::Response, State};
use std::{io::Cursor, path::Path};

#[get("/upload/thumbnail?<file>&<w>&<h>&<blur>&<rotate>")]
pub fn thumbnail(
  file: String,
  w: Option<i32>,
  h: Option<i32>,
  blur: Option<f32>,
  rotate: Option<i32>,
  conf: State<Conf>,
) -> Response<'static> {
  let ref upload_dir = conf.upload_dir;
  let path = Path::new(upload_dir).join(&file);
  let mut f = image::open(&path).unwrap();
  let format = ImageFormat::from_path(&path).unwrap();
  if w != None && h != None {
    f = f.thumbnail(w.unwrap() as u32, h.unwrap() as u32);
  }
  if let Some(sigma) = blur {
    f = f.blur(sigma);
  }
  match rotate {
    Some(90) => {
      f = f.rotate90();
    }
    Some(180) => {
      f = f.rotate180();
    }
    Some(270) => {
      f = f.rotate270();
    }
    _ => (),
  }
  let mut d: Cursor<Vec<u8>> = Cursor::new(Vec::new());
  f.write_to(&mut d, format).unwrap();
  let content_type = match format {
    ImageFormat::Jpeg => ContentType::JPEG,
    ImageFormat::Bmp => ContentType::BMP,
    ImageFormat::Gif => ContentType::GIF,
    ImageFormat::Ico => ContentType::Icon,
    ImageFormat::Png => ContentType::PNG,
    ImageFormat::WebP => ContentType::WEBP,
    _ => ContentType::Plain,
  };
  Response::build()
    .header(content_type)
    .sized_body(d)
    .finalize()
}
