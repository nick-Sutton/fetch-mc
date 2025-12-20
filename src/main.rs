use anyhow::Result;
use clap::Parser;

mod api;
mod config;

#[derive(Parser)]
#[command(name = "fetch-mc")]
#[command(about = "Download MC mods from Modrinth", long_about = None)]
struct Cli {
    #[arg(short, long)]
    mc_version: Option<String>,
    
    #[arg(short, long, default_value = "config.toml")]
    config_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create CLI parser
    let cli = Cli::parse();
    
    // Parse config file
    println!("Loading config from: {}", cli.config_path);
    let config = config::parse_config(&cli.config_path)?;
    
    // Create Modrinth Client
    let mod_client = api::ModrinthClient::new();

    // Convert Option<String> to Option<&str>
    let mc_version = cli.mc_version.as_deref();
    
    // Fetch mods from Modrinth
    if let Some(mod_list) = config.mods {
        println!("Fetching {} mods...", mod_list.len());
        for mc_mod in mod_list {
            println!("Downloading mod: {}", mc_mod);    
            let versions = mod_client.fetch_versions(&mc_mod, mc_version).await?;
            
            if let Some(latest) = versions.first() {
                println!("  Found version: {}", latest.name);
                // Download file
                // download_file(&latest.files[0].url, &format!("mods/{}", latest.files[0].filename)).await?;
            } else {
                println!("  No compatible version found");
            }
        }
    } else {
        println!("No Mods to Fetch.");
    }
    
    // Fetch resourcepacks from Modrinth
    if let Some(rp_list) = config.resourcepacks {
        println!("Fetching {} Resource Packs...", rp_list.len());
        for rp in rp_list {
            println!("Downloading Resource Pack: {}", rp);
            
            let versions = mod_client.fetch_versions(&rp, mc_version).await?;
            
            if let Some(latest) = versions.first() {
                println!("  Found version: {}", latest.name);
                // TODO: Download the file
                // download_file(&latest.files[0].url, &format!("mods/{}", latest.files[0].filename)).await?;
            } else {
                println!("  No compatible version found");
            }
        }
    } else {
        println!("No Mods to Fetch.");
    }
    
    // Fetch Shaders from Modrinth
    if let Some(shader_list) = config.shaders {
        println!("Fetching {} Shaders...", shader_list.len());
        for shader in shader_list {
            println!("Downloading Shader: {}", shader);
            
            let versions = mod_client.fetch_versions(&shader, mc_version).await?;
            
            if let Some(latest) = versions.first() {
                println!("  Found version: {}", latest.name);
                // TODO: Download the file
                // download_file(&latest.files[0].url, &format!("mods/{}", latest.files[0].filename)).await?;
            } else {
                println!("  No compatible version found");
            }
        }
    } else {
        println!("No Shaders to Fetch.");
    }
    
    println!("Done!");
    Ok(())
}