use crate::api::{APIError, APIResult, Conf};
use base64;
use rocket::{http::ContentType, Data, Route, State};
use rocket_multipart_form_data::{
  mime, FileField, MultipartFormData, MultipartFormDataError, MultipartFormDataField,
  MultipartFormDataOptions, SingleFileField,
};
use std::{collections::HashMap, fs, path::Path, time::SystemTime};

fn save_upload<'a: 'b, 'b>(
  file: &'b SingleFileField,
  upload: &'a str,
) -> Result<(String, String), String> {
  let file_name = file.file_name.as_ref().unwrap();
  let ext_name = Path::new(file_name).extension().unwrap();
  let now = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_millis();
  let dest_name = Path::new(file_name)
    .with_file_name(base64::encode(file_name.to_string() + &now.to_string()))
    .with_extension(ext_name);
  let dest_path = Path::new(upload).join(&dest_name);
  fs::copy(&file.path, &dest_path).map_err(|_| "copy to dest path failed".to_string())?;
  Ok((
    file_name.to_string(),
    dest_name.to_str().unwrap().to_string(),
  ))
}

#[post("/upload", format = "multipart", data = "<data>")]
pub fn upload(content_type: &ContentType, data: Data, conf: State<Conf>) -> APIResult {
  let mut options = MultipartFormDataOptions::new();
  options.allowed_fields.push(
    MultipartFormDataField::file("file")
      .size_limit(1024 * 1024 * 5) // 5mb
      .content_type_by_string(Some(mime::STAR_STAR))
      .unwrap(),
  );
  let multipart_form_data =
    MultipartFormData::parse(content_type, data, options).map_err(|e| match e {
      MultipartFormDataError::DataTooLargeError(filed) => {
        APIError::from(format!("file {} is too large", filed))
      }
      MultipartFormDataError::DataTypeError(field) => {
        APIError::from(format!("file {} type is unacceptable", field))
      }
      MultipartFormDataError::NotFormDataError => {
        APIError::from("content type is not `multipart/form-data`")
      }
      MultipartFormDataError::BoundaryNotFoundError => {
        APIError::from("the multipart form data is incorrect")
      }
      MultipartFormDataError::IOError(err) => APIError::from(format!("{:?}", err)),
      MultipartFormDataError::FromUtf8Error(err) => APIError::from(format!("{:?}", err)),
    })?;
  println!("files {:?}", multipart_form_data.files);
  let file = multipart_form_data.files.get("file").unwrap();
  match file {
    FileField::Single(file) => {
      let (file_name, dest_name) =
        save_upload(file, &conf.upload).map_err(|e| APIError::from(e))?;
      return Ok(response!({ file_name: dest_name }));
    }
    FileField::Multiple(files) => {
      let mut ret: HashMap<String, String> = HashMap::new();
      for file in files {
        let (file_name, dest_name) =
          save_upload(file, &conf.upload).map_err(|e| APIError::from(e))?;
        ret.insert(file_name, dest_name);
      }
      return Ok(response!(ret));
    }
  }
}

