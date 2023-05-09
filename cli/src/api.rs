use reqwest::Client;
use std::error::Error;
use std::path::Path;

pub const REGISTRY_URL: &str = "https://api.unchained.watch/abi";

pub fn upload_file(
    client: &Client,
    file_path: &Path,
    exclude: &Option<Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    if let Some(exclude_patterns) = exclude {
        for exclude_pattern in exclude_patterns {
            if glob::Pattern::new(exclude_pattern)?.matches(file_path.to_str().unwrap()) {
                println!("Dont upload {}", file_path.to_str().unwrap());
                return Ok(());
            }
        }
    }

    println!("Upload {}", file_path.to_str().unwrap());

    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    // let file_content = fs::read(file_path)?;

    // client.post(REGISTRY_URL)
    //     .header("Content-Type", "application/octet-stream")
    //     .header("Content-Disposition", format!("attachment; filename=\"{}\"", file_name))
    //     .body(file_content)
    //     .send()?;

    Ok(())
}
