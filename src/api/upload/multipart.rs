use multipart::server::Multipart;
use rocket::Data;
use std::{
  env, fs,
  io::{Cursor, Read, Write},
  path::Path,
  time::SystemTime,
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct TextPart {
  pub key: String,
  pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilePart {
  pub path: String,
  pub filename: String,
  pub extname: String,
  pub size: u64,
}

impl FilePart {
  fn normalize_name(filename: &str) -> String {
    let ext_name = Path::new(filename).extension().unwrap();
    let now = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap()
      .as_millis();
    Path::new(filename)
      .with_file_name(base64::encode(filename.to_string() + &now.to_string()))
      .with_extension(ext_name)
      .to_str()
      .unwrap()
      .to_string()
  }
  pub fn save(self, p: &Path) -> Result<(String, FilePart), String> {
    let filename = FilePart::normalize_name(&self.filename);
    let s = Path::join(p, &filename);
    fs::copy(Path::new(&self.path), &s).map_err(|e| e.to_string())?;
    let mut file_part = self.clone();
    file_part.filename = filename;
    Ok((self.filename.clone(),file_part))
  }
}

impl Drop for FilePart {
  fn drop(&mut self) {
    let path = Path::new(&self.path);
    if path.is_file() {
      fs::remove_file(path).unwrap();
    }
  }
}

pub struct MultipartParts {
  pub files: Vec<FilePart>,
  pub texts: Vec<TextPart>,
}

pub fn handle_multipart(
  content_type: &str,
  data: Data,
  file_size_limit: u64,
  file_type: &str,
) -> Result<MultipartParts, String> {
  let idx = content_type.find("boundary=").expect("no boundary");
  let boundary = &content_type[(idx + "boundary=".len())..];
  let mut d = Vec::new();
  data.stream_to(&mut d).expect("unable to read");

  let mut multipart = Multipart::with_body(Cursor::new(d), boundary);
  let mut texts = Vec::new();
  let mut files = Vec::new();
  let mut buffer = [0u8; 4096];
  let mut err_out: Option<String> = None;
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
              err_out = Some(format!("{:?}", err));
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
            err_out = Some("data can not read as UTF-8".to_string());
            return;
          }
        };
        texts.push(TextPart {
          key: entry.headers.name.to_string(),
          value: text,
        });
      } else {
        let filename = entry.headers.filename.clone().unwrap();
        let ext_name = Path::new(&filename).extension().unwrap();
        let allowed_file_type: Vec<&str> = file_type.split(",").collect();
        if allowed_file_type.contains(&ext_name.to_str().unwrap().trim_start_matches(".")) == false
        {
          err_out = Some(format!("file {} has unacceptable type", &filename));
          return;
        }
        let target_path = Path::join(&tmp_dir, &filename);
        let mut file = match fs::File::create(&target_path) {
          Ok(f) => f,
          Err(err) => {
            err_out = Some(format!("{:?}", err));
            return;
          }
        };
        let mut sum_c = 0u64;
        loop {
          let c = match data.read(&mut buffer) {
            Ok(c) => c,
            Err(err) => {
              try_delete(&target_path);
              err_out = Some(format!("{:?}", err));
              return;
            }
          };
          if c == 0 {
            break;
          }
          sum_c += c as u64;
          if sum_c > file_size_limit {
            try_delete(&target_path);
            err_out = Some(format!("file {} is too large", &filename));
            return;
          }
          match file.write(&buffer[..c]) {
            Ok(_) => (),
            Err(err) => {
              try_delete(&target_path);
              err_out = Some(format!("{:?}", err));
              return;
            }
          }
        }
        files.push(FilePart {
          path: target_path.to_str().unwrap().to_string(),
          filename: entry.headers.filename.clone().unwrap(),
          extname: ext_name.to_str().unwrap().to_string(),
          size: sum_c,
        })
      }
    })
    .unwrap();
  if let Some(failed) = err_out {
    return Err(failed);
  }
  Ok(MultipartParts { texts, files })
}

#[inline]
fn try_delete<P: AsRef<Path>>(path: P) {
  if fs::remove_file(path.as_ref()).is_err() {}
}
