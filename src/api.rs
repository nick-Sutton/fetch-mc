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
}