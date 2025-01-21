// 1-ZIP ALL THE FILES
// 2-CHECK FILE SIZE
// 3-GET PRESIGNED URL
// 4-IF > 5MB DO MULTIPART IF NOT DO SINGLE PART
use anyhow::{Context, Error};
use dioxus::logger::tracing::info;
use dioxus::logger::tracing::warn;
use serde::Deserialize;
use std::io::{Cursor, Write};

use zip::{
    write::{ExtendedFileOptions, FileOptions},
    CompressionMethod, ZipWriter,
};

#[derive(Debug, Clone)]
pub struct UploadFile {
    pub file_name: String,
    pub file_size: u64,
    pub file_contents: Vec<u8>,
}

#[derive(Deserialize)]
pub struct PresignedUrlResponse {
    // #[serde(rename = "Presigned URL")]
    pub url: String,
}

pub(crate) async fn upload_plans(
    upload_files: Vec<UploadFile>,
    total_file_size: f64,
) -> Result<(), Error> {
    info!("Beginning upload process");
    // Zip the files in memory
    let zipped_file = zip_files_in_memory(upload_files, total_file_size)
        .context("Failed to zip files in memory")?;
    info!("Files Zipped, presigning ...");
    // Get the presigned URL
    let presigned = get_presigned_url()
        .await
        .context("Failed to fetch presigned URL")?;

    info!("Presigned, starting s3....");
    let client = reqwest::Client::new();
    let s3_response = client
        .put(&presigned.url)
        // .header("Content-Type", "application/zip") // Explicitly set the content type
        .body(zipped_file) // Use zip_data here
        .send()
        .await
        .context("Failed to send request to S3")?;

    info!("S3 uploaded waiting on status....");
    let status = s3_response.status();
    if status.is_success() {
        Ok(())
    } else {
        let error_body = s3_response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read response body".to_string());
        Err(anyhow::anyhow!(
            "Failed to upload file. Status: {}, Body: {}",
            status,
            error_body
        ))
    }
}

// ZIP CRATE CREATES THE FILE SIZE TO BE WAY BIGGER. FOR INSTANCE PDF SIZES WERE 35MB
// AND THE INCREASED TO 67MB SO ITS NOT GOOD FOR INCREASE SIZES
pub(crate) fn zip_files_in_memory(
    upload_files: Vec<UploadFile>,
    total_file_size: f64,
) -> Result<Vec<u8>, Error> {
    let mut buffer = Cursor::new(Vec::new());
    {
        let mut zip = ZipWriter::new(&mut buffer);

        for pdf in upload_files.iter() {
            let file_size_mb = pdf.file_contents.len() as f64 / 1024.0 / 1024.0;
            info!(
                "Adding file: {}, size before compression: {:.2} MB",
                pdf.file_name, file_size_mb
            );
            //STORED GIVES LOSSLESS COMPRESSION
            let options = FileOptions::<ExtendedFileOptions>::default()
                .compression_method(CompressionMethod::Stored)
                .unix_permissions(0o755);

            let pdf_content = &pdf.file_contents;
            zip.start_file(&pdf.file_name, options)
                .context("Failed to start zipping")?;
            zip.write_all(pdf_content)
                .context("Cannot write zip file")?;

            // let current_zip_size_mb = buffer.get_ref().len() as f64 / 1024.0 / 1024.0;
        }
        zip.finish().context("Failed to finish zip")?;
    }

    let zip_data = buffer.into_inner();
    let zip_size_bytes = zip_data.len();
    let zip_size_mb = zip_size_bytes as f64 / 1024.0 / 1024.0;

    info!(
        "Before Zip: {} Mb After Zip: ({} MB)",
        total_file_size, zip_size_mb
    );

    Ok(zip_data)
}

// GET THE PRESIGNED URL FROM THE BACKEND
pub(crate) async fn get_presigned_url() -> Result<PresignedUrlResponse, Error> {
    let lambda_url = format!(
        "http://localhost:9000/lambda-url/lambda_upload_plans/?bucket={}&key={}",
        "dioxus-upload", "files.zip"
    );
    // let lambda_url1 = format!(
    //     "http://localhost:9000/lambda-url/upload_files/?bucket={}&key={}",
    //     "dioxus-upload", "files.zip"
    // );
    warn!("Calling for presigned url");
    let response = reqwest::get(&lambda_url)
        .await
        .context("Error making GET request")?;
    info!("Raw Response Metadata: {:?}", response);

    // Ensure the response is successful
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Request failed with status: {}",
            response.status()
        ));
    }

    // let response = reqwest::get(&lambda_url).await;
    // info!("Raw Response Metadata: {:?}", response);

    let json: PresignedUrlResponse = response.json().await.context("Error converting to json")?;
    info!("Json Response: {:?}", json.url);
    Ok(json)
}
