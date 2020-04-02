use multipart::server::Multipart;
use rocket::{
  data::{self, FromDataSimple},
  http::Status,
  Data, Outcome,
  Outcome::*,
  Request,
};
use std::{
  env, fs,
  io::{Cursor, Read, Write},
  path::Path,
};

#[derive(Debug)]
pub struct TextPart {
  pub key: String,
  pub value: String,
}

#[derive(Debug)]
pub struct FilePart {
  pub name: String,
  pub path: String,
  pub filename: String,
}

impl FilePart {
  pub fn save(&self, p: &Path) {
    let s = Path::join(p, &self.filename);
    fs::copy(Path::new(&self.path), &s).unwrap();
  }
}

impl Drop for FilePart {
  fn drop(&mut self) {
    fs::remove_file(Path::new(&self.path)).unwrap();
  }
}

#[derive(Debug)]
pub struct MultipartData {
  pub texts: Vec<TextPart>,
  pub files: Vec<FilePart>,
}

impl<'a> FromDataSimple for MultipartData {
  type Error = String;
  fn from_data(request: &Request, data: Data) -> data::Outcome<Self, String> {
    let content_type = request
      .headers()
      .get_one("Content-Type")
      .expect("no content-type");
    let idx = content_type.find("boundary=").expect("no boundary");
    let boundary = &content_type[(idx + "boundary=".len())..];
    let mut d = Vec::new();
    data.stream_to(&mut d).expect("unable to read");

    let mut multipart = Multipart::with_body(Cursor::new(d), boundary);
    let mut texts = Vec::new();
    let mut files = Vec::new();
    let mut buffer = [0u8; 4096];
    let mut err_out: Option<Outcome<_, (Status, _), _>> = None;
    let tmp_dir = env::temp_dir();

    multipart
      .foreach_entry(|entry| {
        let mut data = entry.data;
        if entry.headers.filename == None {
          let mut text_buf = Vec::new();
          loop {
            let c = match data.read(&mut buffer) {
              Ok(c) => c,
              Err(err) => {
                err_out = Some(Failure((Status::UnprocessableEntity, format!("{:?}", err))));
                return;
              }
            };
            if c == 0 {
              break;
            }
            text_buf.extend_from_slice(&buffer[..c]);
          }
          let text = match String::from_utf8(text_buf) {
            Ok(s) => s,
            Err(_) => {
              err_out = Some(Failure((
                Status::UnprocessableEntity,
                "data can not read as UTF-8".into(),
              )));
              return;
            }
          };
          texts.push(TextPart {
            key: entry.headers.name.to_string(),
            value: text,
          });
        } else {
          let filename = entry.headers.filename.clone().unwrap();
          let target_path = Path::join(&tmp_dir, &filename);
          let mut file = match fs::File::create(&target_path) {
            Ok(f) => f,
            Err(err) => {
              err_out = Some(Failure((Status::InternalServerError, format!("{:?}", err))));
              return;
            }
          };
          let mut _sum_c = 0u64;
          loop {
            let c = match data.read(&mut buffer) {
              Ok(c) => c,
              Err(err) => {
                try_delete(&target_path);
                err_out = Some(Failure((Status::UnprocessableEntity, format!("{:?}", err))));
                return;
              }
            };
            if c == 0 {
              break;
            }
            _sum_c += c as u64;
            match file.write(&buffer[..c]) {
              Ok(_) => (),
              Err(err) => {
                try_delete(&target_path);
                err_out = Some(Failure((Status::InternalServerError, format!("{:?}", err))));
                return;
              }
            }
          }
          files.push(FilePart {
            name: entry.headers.name.to_string(),
            path: String::from(tmp_dir.to_str().unwrap()) + &filename,
            filename: entry.headers.filename.clone().unwrap(),
          })
        }
      })
      .unwrap();
    if let Some(failed) = err_out {
      return failed;
    } else {
      return Outcome::Success(MultipartData { texts, files });
    }
  }
}

#[inline]
fn try_delete<P: AsRef<Path>>(path: P) {
  if fs::remove_file(path.as_ref()).is_err() {}
}
