use clap::Parser;
use colored::*;
use chrono::{DateTime, Local};
use std::path::PathBuf;
use std::io;
use walkdir::WalkDir;

fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the directory to inspect
    #[arg(default_value = ".")]
    path: PathBuf,

    /// The depth of subdirectories to search
    #[arg(short, long, default_value_t = 1)]
    depth: usize,
}

fn main() {
    let args = Args::parse();

    if !args.path.is_dir() {
        eprintln!("{}", "Error: The specified path is not a directory.".red());
        return;
    }

    match display_directory_info(&args.path, args.depth) {
        Ok(_) => (),
        Err(e) => eprintln!("{} {}", "Error:".red(), e),
    }
}

fn display_directory_info(path: &PathBuf, max_depth: usize) -> io::Result<()> {
    println!();
    println!("{} '{}'", "Inspecting directory:".bold().cyan(), path.display());
    println!("{} {}", "Search depth:".bold().cyan(), max_depth);
    println!("{}", "--------------------------------------------------".dimmed());

    let mut total_size = 0;
    let mut file_types: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    let mut folder_count = 0;

    for entry in WalkDir::new(path).max_depth(max_depth) {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let entry_path = entry.path();

        if entry.depth() == 0 { // Skip the root directory itself
            continue;
        }

        let depth = entry.depth();
        let indent = "  ".repeat(depth.saturating_sub(1));

        if metadata.is_dir() {
            folder_count += 1;
            println!("{}{}{}", indent, "📁 ".yellow(), entry_path.file_name().unwrap().to_str().unwrap().bold().blue());
        } else {
            let size = metadata.len();
            total_size += size;
            let file_name = entry_path.file_name().unwrap().to_str().unwrap();
            let extension = entry_path.extension().and_then(|s| s.to_str()).unwrap_or("other");
            *file_types.entry(extension.to_string()).or_insert(0) += 1;

            let created: DateTime<Local> = DateTime::from(metadata.created()?);
            let permissions = if metadata.permissions().readonly() { "ro" } else { "rw" };

            println!(
                "{}{}{} {} {} {} {}",
                indent,
                "📄 ".green(),
                file_name.white(),
                format_size(size).bright_yellow(),
                format!("({})", permissions).dimmed(),
                created.format("%Y-%m-%d %H:%M").to_string().dimmed(),
                format!("[{}]", extension).cyan()
            );
        }
    }

    println!("{}", "--------------------------------------------------".dimmed());
    println!("{}: {}", "Total Size".bold().cyan(), format_size(total_size).bright_green());
    println!("{}: {}", "Total Folders".bold().cyan(), folder_count.to_string().bright_green());
    println!("{}:", "File Types".bold().cyan());
    for (ext, count) in file_types.iter() {
        println!("  - {}: {}", ext.cyan(), count.to_string().bright_green());
    }
    println!();

    Ok(())
}
