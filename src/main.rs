use google_ai_rs::{Client, AsSchema};
//use google_ai_rs::Tool;
//use serde::*;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use tokio::time::{sleep, Duration};
// use gcp_bigquery_client::Client as BQClient;
// use gcp_bigquery_client::model::table_data_insert_all_request::TableDataInsertAllRequest;

use clap::Parser;
mod definitions;
use definitions::args::Args;
use definitions::resp_types::*;

#[derive(Debug)]
struct ModelQuota {
    model_name: String,
    rpm: f32,
}

fn read_quotas_simple(path: &str) -> Vec<ModelQuota> {
    let mut quotas = Vec::new();

    // 1. Open the file
    let file = File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);

    // 2. Iterate over each line
    for line in reader.lines() {
        if let Ok(content) = line {
            // 3. Split by comma
            let parts: Vec<&str> = content.split(',').collect();
            
            if parts.len() == 2 {
                let name = parts[0].trim().to_string();
                let rpm = parts[1].trim().parse::<f32>().unwrap_or(0.0);
                
                quotas.push(ModelQuota { model_name: name, rpm });
            }
        }
    }

    quotas
}

async fn create_model_response<T>(
    client: &Client,
    model_list: &[ModelQuota],
    mut model_position: usize,
    isin: &str,
    prompt: &str,
    retries: usize,
) -> Result<(T, f32), Box<dyn std::error::Error>>
where
    T: AsSchema + serde::de::DeserializeOwned + std::fmt::Debug + std::marker::Send,
{
    let mut model = client.typed_model::<T>(&model_list[model_position].model_name);
    let mut rpm = model_list[model_position].rpm;
    let mut attempts = 0;

    loop {
        log::debug!("Prompt: {prompt}");

        match model.generate_content(prompt).await {
            Ok(res) => return Ok((res, rpm)),
            Err(e) => {
                attempts += 1;
                if attempts > retries {
                    log::error!("Final failure for ISIN {}: {}", isin, e);
                    return Err(e.into());
                }

                let wait_time = attempts as u64 * 5;
                log::warn!("Error fetching {}: {:.100}. Retry {}/{} in {}s...", isin, &e, attempts, retries, wait_time);
                sleep(Duration::from_secs(wait_time)).await;

                let error_msg = e.to_string();
                let is_rate_limited = error_msg.contains("ResourceExhausted") || error_msg.contains("429");
                if model_list.len() > 1 && is_rate_limited {
                    model_position = (model_position + 1) % model_list.len();
                    log::info!("Rate limit hit. Switching to model: {}", &model_list[model_position].model_name);
                    model = client.typed_model::<T>(&model_list[model_position].model_name);
                    rpm = model_list[model_position].rpm;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    env_logger::init();

    println!("Configuration: {:?}, Log Level: {}", args, std::env::var("RUST_LOG").unwrap_or("ERROR".to_string()));

    // check if args.isin_path is default <ISIN>.md then replace <ISIN> with the actual ISIN value
    let content_filepath = &args.isin_path.replace("<ISIN>", &args.isin.to_ascii_uppercase());
    let content = std::fs::read_to_string(&content_filepath)?;
    //     .map_err(|_| "Please provide a valid text file containing the certificate description")?;
    // let bytes = std::fs::read(&content_filepath)?;
    // let content2 = String::from_utf8_lossy(&bytes);

    let model_list: Vec<ModelQuota> = match args.model_list_path {
        Some(path) => {
            log::debug!("Loading models from: {}", path);
            read_quotas_simple(&path)
        },
        None => {
            log::debug!("No model list provided. Using default: {}", &args.model);
            vec![ModelQuota {
                model_name: args.model,
                rpm: args.rpm,
            }]
        }
    };

    let output_dir = &args.output_dir;
    // Ensure directory exists
    std::fs::create_dir_all(output_dir)?;

    let g_api_key = std::env::var("G_API_KEY").map_err(|_| "Configuration error, please contact service administrator.".to_string())?;
    let client = Client::new(g_api_key).await?;
    
    let model_position = args.model_pos;
    let isin = &args.isin.to_ascii_uppercase();
    let prompt = match args.resp_type.as_str() {
        "tickers-only" => format!(
            "what are the underlying stocks under the certificate {isin} based on {content}? please do not consider other information such as certificate details!"
        ),
        "details-only" => format!(
            "what is the information about the certificate {isin} based on {content}? Please do not consider underlying stocks information nor issuer details!"
        ),
        "issuer-only" => format!(
            "what is the information about the issuer of the certificate {isin} based on {content}? Please do not consider underlying stocks information nor certificate details!"
        ),
        _ => format!(
            "what is the information about the certificate {isin} based on {content}? Please add underlying stocks information as well!"
        ),
    };

    let (response, rpm) = create_model_response::<CertificateTickersResponse>(
        &client,
        &model_list,
        model_position,
        &isin,
        &prompt,
        args.retries,
    ).await?;
      
    log::debug!("Response: {:?}", response);

    let file_name = format!("{}.json", isin);
    let full_path = std::path::Path::new(output_dir).join(file_name);

    // Serialize and write
    let json_string = serde_json::to_string_pretty(&response)?;
    std::fs::write(&full_path, &json_string)?;
    log::debug!("File saved to: {:?}", full_path);

    if let Some(details) = &response.details {
        let file_name = format!("{}-details.json", isin);
        let full_path = std::path::Path::new(output_dir).join(file_name);

        // Serialize and write
        let json_string = serde_json::to_string_pretty(details)?;
        std::fs::write(&full_path, &json_string)?;
        log::debug!("Details information saved to: {:?}", full_path);
    }
    if let Some(issuer) = &response.issuer {
        let file_name = format!("{}-issuer.json", isin);
        let full_path = std::path::Path::new(output_dir).join(file_name);

        // Serialize and write
        let json_string = serde_json::to_string_pretty(issuer)?;
        std::fs::write(&full_path, &json_string)?;
        log::debug!("Issuer information saved to: {:?}", full_path);
    }

    // create a ndjson file to store underlyings if required
    if let Some(stocks) = &response.underlyings && args.output_format == "ndjson" {
        let ndj_file_name = format!("{}-tickers.json", isin);
        let ndj_full_path = std::path::Path::new(output_dir).join(ndj_file_name);
        let mut file = File::create(&ndj_full_path)?;   
            for stock in stocks {
                log::debug!("Writing json to {:?}...", &ndj_full_path);
                // ndJSON is 1 file containing multiple JSON objects, each in a new line
                serde_json::to_writer(&mut file, &stock).unwrap();
                // add a new line after each JSON object
                file.write_all(b"\n").unwrap();
            }
    }
    log::info!("{}, OK", isin);

    if args.wait {
        let ave_wait = 60.0 / rpm + 0.5;  
        log::debug!("Waiting {} seconds before next request...", ave_wait);
        sleep(Duration::from_secs(ave_wait as u64)).await;
    }

    Ok(())
}