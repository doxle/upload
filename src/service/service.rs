// 1-ZIP ALL THE FILES
// 2-CHECK FILE SIZE
// 3-GET PRESIGNED URL
// 4-IF > 5MB DO MULTIPART IF NOT DO SINGLE PART
use anyhow::{Context, Error, Result};
use dioxus::logger::tracing::error;
use dioxus::logger::tracing::info;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::io::{Cursor, Write};

use zip::{
    write::{ExtendedFileOptions, FileOptions},
    CompressionMethod, ZipWriter,
};

#[derive(Serialize, Debug)]
struct ETag {
    part_number: usize,
    etag: String,
}

#[derive(Debug, Clone)]
pub struct UploadFile {
    pub file_name: String,
    pub file_size: u64,
    pub file_contents: Vec<u8>,
}

#[derive(Deserialize)]
pub struct PresignedResponse {
    // #[serde(rename = "Presigned URL")]
    pub urls: Vec<String>,
    pub upload_id: Option<String>,
}

pub(crate) async fn upload_plans(
    upload_files: Vec<UploadFile>,
    total_file_size: f64,
) -> Result<(), Error> {
    info!("Beginning upload process");
    // Zip the files in memory
    let zipped_result = zip_files_in_memory(upload_files, total_file_size)
        .context("Failed to zip files in memory")?;
    let zipped_file = zipped_result.0;
    let zip_size = zipped_result.1;
    info!("Files Zipped, presigning ...");

    // Get the presigned URL
    let presigned_response = get_presigned_url(zip_size)
        .await
        .context("Failed to fetch presigned URL")?;

    let presigned_urls = presigned_response.urls;

    if presigned_urls.len() == 1 {
        single_part_upload(zipped_file, presigned_urls).await
    } else {
        let upload_id = presigned_response.upload_id;
        match upload_id {
            None => Err(anyhow::anyhow!(
                "No upload ID returned for multipart upload"
            )),
            Some(upload_id) => multi_part_upload(upload_id, zipped_file, presigned_urls).await,
        }
    }
}

//SINGLE FILE UPLOAD
pub(crate) async fn single_part_upload(
    zipped_file: Vec<u8>,
    presigned_urls: Vec<String>,
) -> Result<(), Error> {
    // Handle single URL (less than 5MB)
    info!("Single presigned URL received, starting direct upload...");
    let url = &presigned_urls[0];

    let client = reqwest::Client::new();
    let s3_response = client
        .put(url)
        // .header("Content-Type", "application/zip") // Explicitly set the content type
        .body(zipped_file) // Use zip_data here
        .send()
        .await
        .context("Failed to upload file to S3")?;

    info!("S3 uploaded waiting on status....");
    let status = s3_response.status();
    if status.is_success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Failed to upload file. Status: {}, Body: {}",
            status,
            s3_response
                .text()
                .await
                .unwrap_or("Failed to read response body".into())
        ))
    }
}

//NEED TO COLLECT ETAGS AFTER EVERY CHUNK AND THEN COMPLETE THE UPLOAD
// OR WE CAN GET 200 AND NOTHING WILL BE UPLOADED TO S3 TILL WE COMPLETE_MULTIPART_UPLOAD
pub(crate) async fn multi_part_upload(
    upload_id: String,
    zip_file: Vec<u8>,
    presigned_urls: Vec<String>,
) -> Result<(), Error> {
    // let mut etags = vec![];
    info!("Multiple presigned URLs received, starting chunked upload...");
    let mut offset: usize = 0;
    let client = reqwest::Client::new();
    let chunk_size: usize = 5 * 1024 * 1024;
    let mut etags: Vec<ETag> = vec![];

    for (index, url) in presigned_urls.iter().enumerate() {
        info!("Current Index: {}", index);
        info!("Current URL: {}", url);
        let chunk_end = std::cmp::min(offset + chunk_size, zip_file.len());
        let current_chunk = &zip_file[offset..chunk_end];
        // Upload the current chunk to the presigned URL
        info!("Current Chunk: {:?}", current_chunk.len());

        // let complete_multipart_upload = CompletedMultipartUpload::builder()
        //     .set_parts(Some())
        //     .build();

        let response = client
            .put(url)
            .body(current_chunk.to_vec()) // Convert the chunk to Vec<u8>
            .send()
            .await
            .expect(&format!("Failed to upload chunk {}", index + 1));

        let status = response.status();
        info!("Status Code: {}", status);
        if !status.is_success() {
            // println!("Failed to upload chunk {}", index + 1);
            return Err(anyhow::anyhow!(
                "Failed to upload  with Upload Chunk {}, Status: {}, Body: {}",
                index,
                status,
                response
                    .text()
                    .await
                    .unwrap_or("Failed to read response body".into())
            ));
        }
        // WE NEED TO COLLECT ETAGS AFTER EVERY CHUNK AND THEN COMPLETE THE UPLOAD
        else {
            info!("Current Chunk is uploaded: {}", index);
            if let Some(etag) = response.headers().get("etag") {
                // etags.push((index, etag.to_str().unwrap().to_string()));
                etags.push({
                    ETag {
                        part_number: index + 1,
                        etag: etag.to_str().unwrap().to_string(),
                    }
                });
            }
            // info!("Headers : {:?}", headers);
        }
        // MOVE THE STARTING POINT TO THE CURRENT CHUNK TO START THE NEXT STARTING INDEX
        offset = chunk_end;
    }
    for etag in etags.iter() {
        info!("ETAGS: {:?}", etag);
    }

    let client = reqwest::Client::new();
    //MAKE CALL TO BE AND PASS THE ETAGS
    let lambda_etags_url = format!("http://localhost:9000/lambda-url/lambda_upload_plans/");
    let etags_payload = json!({
        "etags": etags,
        "upload_id": upload_id,
        "bucket": "dioxus-upload",
        "key": "files.zip"
    });

    let response = client
        .post(&lambda_etags_url)
        .json(&etags_payload)
        .send()
        .await
        .context("Error making POST request")?;

    // Access and print headers
    let headers = response.headers();
    for (key, value) in headers {
        println!("Header: {} => {:?}", key, value);
    }

    // Handle the response
    if response.status().is_success() {
        info!("ETags successfully sent to backend!");
    } else {
        error!(
            "Failed to send ETags. Status: {}, Body: {}",
            response.status(),
            response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read response body".to_string())
        );
    }

    Ok(())
}

// ZIP CRATE CREATES THE FILE SIZE TO BE WAY BIGGER. FOR INSTANCE PDF SIZES WERE 35MB
// AND THE INCREASED TO 67MB SO ITS NOT GOOD FOR INCREASE SIZES

// GET THE PRESIGNED URL FROM THE BACKEND
pub(crate) async fn get_presigned_url(size_in_bytes: usize) -> Result<PresignedResponse, Error> {
    info!("size: {}", size_in_bytes);
    let lambda_url = format!(
        "http://localhost:9000/lambda-url/lambda_upload_plans/?bucket={}&key={}&size={}",
        "dioxus-upload", "files.zip", size_in_bytes
    );
    // let lambda_url1 = format!(
    //     "http://localhost:9000/lambda-url/upload_files/?bucket={}&key={}",
    //     "dioxus-upload", "files.zip"
    // );
    info!("Calling for presigned url");
    let response = reqwest::get(&lambda_url)
        .await
        .context("Error making GET request")?;

    info!("{}", "*".repeat(45));
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

    let json: PresignedResponse = response.json().await.context("Error converting to json")?;
    // info!("Json Response: {:?}", json.urls);
    for url in json.urls.iter() {
        info!("Presigned URL: {}", url);
    }
    info!("{}", "*".repeat(45));
    Ok(json)
}

pub(crate) fn zip_files_in_memory(
    upload_files: Vec<UploadFile>,
    total_file_size: f64,
) -> Result<(Vec<u8>, usize), Error> {
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

    //RETURN THE ZIP FILE AND THE TOTAL SIZE TO WORKOUT SINGLE OR MULTI-PART
    Ok((zip_data, zip_size_bytes))
}
