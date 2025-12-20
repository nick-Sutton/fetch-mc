use std::path::Path;

use anyhow::Result;
use reqwest::{self};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Version {
    pub name: String,
    pub files: Vec<FileInfo>,
}

#[derive(Deserialize, Debug)]
pub struct FileInfo {
    pub url: String,
    pub filename: String,
}

pub struct ModrinthClient {
    client: reqwest::Client,
    base_url: String,
}

impl ModrinthClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.modrinth.com/v2".to_string(), 
        }
    }
    
    pub async fn fetch_versions(&self, project_slug: &str, mc_version: Option<&str>) -> Result<Vec<Version>> {
        let url = format!("{}/project/{}/version", self.base_url, project_slug);
        
        let mut request = self.client.get(&url);
        
        // Add MC version filter if provided
        if let Some(version) = mc_version {
            request = request.query(&[("game_versions", format!(r#"["{}"]"#, version))]);
        }
        
        let versions: Vec<Version> = request
            .send()
            .await?
            .json()
            .await?;
        
        Ok(versions)
    }

    pub async fn download_item(&self, slug: &str, mc_version: Option<&str>, output_dir: &str) -> Result<()> {
        let versions = self.fetch_versions(slug, mc_version).await?;
        
        let latest = versions.first()
            .ok_or_else(|| anyhow::anyhow!("No compatible version found"))?;
        
        println!("  Found version: {}", latest.name);
        
        let file = latest.files.first()
            .ok_or_else(|| anyhow::anyhow!("No files available"))?;
        
        let output_path = Path::new(output_dir).join(&file.filename);
        println!("  Downloading to: {}", output_path.display());
        
        // Download the file
        let response = self.client.get(&file.url).send().await?;
        let bytes = response.bytes().await?;
        std::fs::write(output_path, bytes)?;
        
        Ok(())
    }
}