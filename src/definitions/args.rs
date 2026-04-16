use clap::*;

#[derive(Parser, Debug)]
#[command(version, about = "Digital Posture Gemini Certificate Get Tickers", long_about = "It retrieves stock information related to a given ISIN using Gemini API and saves the response in the specified format.")]
pub struct Args {
    /// ISIN value 
    #[arg(short = 'n', long)]
    pub isin: String,

    /// Response type [certificate|tickers] 
    #[arg(short = 't', long, default_value = "certificate", value_parser = ["certificate", "tickers"])]
    pub resp_type: String,

    /// Content file path 
    #[arg(short = 'i', long="content-filepath", default_value = "<ISIN>.md")]
    pub isin_path: String,

    /// Output file format [json|ndjson|csv] 
    #[arg(short = 'f', long, default_value = "ndjson", value_parser = ["json-only", "ndjson", "csv"])]
    pub output_format: String,

    /// Output dir path
    #[arg(short = 'o', long, default_value = "./")]
    pub output_dir: String,

        /// Gemini retries
    #[arg(short = 'r', long, default_value_t = 3)]
    pub retries: usize,

    /// Gemini model
    #[arg(short = 'm', long, default_value = "gemini-3-flash-preview")]
    pub model: String,

     /// Gemini model RPM
    #[arg(long, default_value_t = 10.0)]
    pub rpm: f32,  

     /// Gemini model list file path (to cycle on models if provided)
    #[arg(short = 'l', long)]
    pub model_list_path: Option<String>,

            /// Gemini model list start position
    #[arg(short = 'p', long, default_value_t = 0)]
    pub model_pos: usize,

    /// Wait to respect RPM
    #[arg(short = 'w', long, default_value_t = true)]
    pub wait: bool, 
}