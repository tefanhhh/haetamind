use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, Error, HttpResponse, Responder};

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("")]
pub async fn save_files(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    for f in form.files {
        let path = format!("./tmp/{}", f.file_name.unwrap());
        f.file.persist(path).unwrap();
    }
    Ok(HttpResponse::Ok())
}
