use anyhow::Result;
use clap::Parser;

mod config;
mod api;

#[derive(Parser)]
#[command(name = "fetch-mc")]
#[command(about = "Download MC mods from Modrinth", long_about = None)]
struct Cli {
    #[arg(short, long)]
    version: Option<String>,
    
    #[arg(short, long, default_value = "mc_config.toml")]
    config_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    println!("Loading config from: {}", cli.config_path);
    let config = config::parse_config(&cli.config_path)?;

    // Create output directories
    std::fs::create_dir_all("mods")?;
    std::fs::create_dir_all("resourcepacks")?;
    std::fs::create_dir_all("shaderpacks")?;
    
    let client = api::ModrinthClient::new();
    let mc_version = cli.version.as_deref();
    
    // Fetch mods
    if let Some(mod_list) = config.mods {
        println!("\nFetching {} mods...", mod_list.len());
        for mc_mod in mod_list {
            println!("Downloading mod: {}", mc_mod);
            
            // Download mod
            match client.download_item(&mc_mod, mc_version, "mods").await {
                Ok(_) => println!("  ✓ Downloaded"),
                Err(e) => println!("  ✗ Failed: {}", e),
            }
        }
    } else {
        println!("No Mods to Fetch.");
    }
    
    // Fetch resource packs
    if let Some(rp_list) = config.resourcepacks {
        println!("\nFetching {} Resource Packs...", rp_list.len());
        for rp in rp_list {
            println!("Downloading Resource Pack: {}", rp);
            
            // Download Resource Pack
            match client.download_item(&rp, mc_version, "resourcepacks").await {
                Ok(_) => println!("  ✓ Downloaded"),
                Err(e) => println!("  ✗ Failed: {}", e),
            }
        }
    } else {
        println!("No Resource Packs to Fetch.");
    }
    
    // Fetch shaders
    if let Some(shader_list) = config.shaders {
        println!("\nFetching {} Shaders...", shader_list.len());
        for shader in shader_list {
            println!("Downloading Shader: {}", shader);
            
            // Download Shader
            match client.download_item(&shader, mc_version, "shaderpacks").await {
                Ok(_) => println!("  ✓ Downloaded"),
                Err(e) => println!("  ✗ Failed: {}", e),
            }
        }
    } else {
        println!("No Shaders to Fetch.");
    }
    
    println!("\n✓ All downloads complete!");
    Ok(())
}