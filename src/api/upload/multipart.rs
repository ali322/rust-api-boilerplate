use multipart::server::Multipart;
use rocket::Data;
use serde::{Deserialize, Serialize};
use std::{
  env, fs,
  io::{Cursor, Read, Write},
  path::Path,
  time::SystemTime,
};

#[derive(Debug)]
pub struct TextPart {
  pub key: String,
  pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilePart {
  pub path: String,
  pub filename: String,
  pub basename: Option<String>,
  pub extname: String,
  pub size: u64,
}

impl FilePart {
  fn normalize_name(&self) -> String {
    let now = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap()
      .as_millis();
    let filename = self.filename.clone();
    let extname = self.extname.clone();
    Path::new(&filename)
      .with_file_name(base64::encode_config(
        filename.to_string() + &now.to_string(),
        base64::URL_SAFE,
      ))
      .with_extension(&extname)
      .to_str()
      .unwrap()
      .to_string()
  }
  pub fn save(self, p: &Path) -> Result<(String, FilePart), String> {
    let origin_name = self.filename.clone();
    let origin_path = self.path.clone();
    let filename = self.normalize_name();
    if !p.is_dir() {
      fs::create_dir_all(p).map_err(|e| e.to_string())?;
    }
    let s = Path::join(p, &filename);
    fs::copy(Path::new(&origin_path), &s).map_err(|e| e.to_string())?;
    let mut file_part = self.clone();
    file_part.filename = filename;
    Ok((origin_name, file_part))
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
        let ext_name = match Path::new(&filename).extension() {
          Some(ext) => ext.to_str().unwrap().to_lowercase(),
          None => {
            err_out = Some(format!("file {} has invalid extension name", &filename));
            return;
          }
        };
        let allowed_file_type: Vec<&str> = file_type.split(",").collect();
        if allowed_file_type.contains(
          &ext_name.trim_start_matches(".")
        ) == false
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
          basename: None,
          extname: ext_name.to_string(),
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
