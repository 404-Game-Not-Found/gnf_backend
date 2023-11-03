use actix_multipart::Multipart;
use actix_web::HttpResponse;
use futures::StreamExt;
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, actix_web::error::Error> {
    while let Some(field) = payload.next().await {
        let mut item = field?;
        /*
        let content_disposition = item.content_disposition();
        let filename = content_disposition.get_filename().unwrap();
         */
        let filename = "test";

        // Adjust the path where you want to save the file
        let filepath = format!("uploads/{}", filename);
        let mut file = File::create(filepath).await.expect("Failed to create file");

        while let Some(chunk) = item.next().await {
            let data = chunk.expect("Failed to read data");
            file.write_all(&data).await.expect("Failed to write data");
        }
    }

    Ok(HttpResponse::Ok().body("File uploaded"))
}
