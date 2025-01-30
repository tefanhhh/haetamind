use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::error::ErrorInternalServerError;
use actix_web::{post, Error, HttpResponse, Responder};
use image::ImageReader;

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
        f.file.persist(&path).unwrap();
        let img = ImageReader::open(path)
            .map_err(ErrorInternalServerError)?
            .decode()
            .map_err(ErrorInternalServerError)?;
        let rgb_image = img.to_rgb8();

        let output_path = format!("./tmp/{}", "compressed.jpg");
        rgb_image
            .save(output_path)
            .map_err(ErrorInternalServerError)?;
    }
    Ok(HttpResponse::Ok())
}
