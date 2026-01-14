use facet::Facet;
use std::path::{Path, PathBuf};
use mtg_gen::*;
use walkdir::WalkDir;

#[derive(Facet, Debug)]
struct Args {
    /// Path to a YAML file or directory containing YAML files
    #[facet(facet_args::positional)]
    input: PathBuf,

    /// Output directory for generated images
    #[facet(facet_args::named, facet_args::short = 'o', default = default_output())]
    output: PathBuf,

    /// DPI for output images (300 or 600)
    #[facet(facet_args::named, default = 300)]
    dpi: u32,
}

fn default_output() -> PathBuf {
    PathBuf::from("./output")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = facet_args::from_std_args()?;

    println!("MTG Card Generator");
    println!("Input: {:?}", args.input);
    println!("Output: {:?}", args.output);
    println!("DPI: {}", args.dpi);

    let renderer = Renderer::new().await?;
    let mut files = Vec::new();

    if args.input.is_file() {
        files.push(args.input.clone());
    } else {
        for entry in WalkDir::new(&args.input) {
            let entry = entry?;
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "yaml" || ext == "yml" {
                        files.push(entry.path().to_path_buf());
                    }
                }
            }
        }
    }

    for file in files {
        match process_file(&file, &args, &renderer).await {
            Ok(_) => println!("Processed {:?}", file),
            Err(e) => eprintln!("Error processing {:?}: {}", file, e),
        }
    }

    Ok(())
}

async fn process_file(file: &Path, args: &Args, renderer: &Renderer) -> anyhow::Result<()> {
    let content = tokio::fs::read_to_string(file).await?;
    let card: Card = facet_yaml::from_str(&content)?;

    let relative_path = if args.input.is_file() {
        Path::new(file.file_name().unwrap())
    } else {
        file.strip_prefix(&args.input)?
    };

    let output_path = args.output.join(relative_path).with_extension("png");
    
    renderer.render_card(&card, &output_path).await?;

    Ok(())
}

