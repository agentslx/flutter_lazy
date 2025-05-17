use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use fs_extra::dir::{self, CopyOptions};

/// Copies a template directory to destination
pub fn copy_template_dir(template_subpath: &str, destination: &Path) -> Result<()> {
    let template_path = std::env::current_exe()?
        .parent()
        .context("Failed to get executable directory")?
        .join("templates")
        .join(template_subpath);
        
    // For development, check local project path as well
    let local_template_path = PathBuf::from("flutter_lazy/templates").join(template_subpath);
    
    let source_path = if template_path.exists() {
        template_path
    } else if local_template_path.exists() {
        local_template_path
    } else {
        return Err(anyhow::anyhow!("Template directory not found: {}", template_subpath));
    };
    
    // Create destination if it doesn't exist
    std::fs::create_dir_all(destination)?;
    
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    
    // Copy directory contents
    if source_path.exists() && source_path.is_dir() {
        dir::copy(source_path, destination, &copy_options)?;
    }
    
    Ok(())
}

/// Copies a template file and replaces placeholders
pub fn copy_template_file(template_path: &str, dest_path: &Path, replacements: &[(&str, &str)]) -> Result<()> {
    // Determine the template path
    let mut template_content = String::new();
    
    let exe_template_path = std::env::current_exe()?
        .parent()
        .context("Failed to get executable directory")?
        .join("templates")
        .join(template_path);
        
    // For development, check multiple local project paths
    let paths_to_check = vec![
        // Path when run from project root
        PathBuf::from("templates").join(template_path),
        // Path when run from inside project directory
        PathBuf::from("flutter_lazy/templates").join(template_path),
        // Current directory path
        PathBuf::from("./templates").join(template_path),
        // Path relative to workspace root
        PathBuf::from("../templates").join(template_path),
    ];
    
    // Try to find the template path
    let mut template_file = None;
    
    // First check the executable path
    if exe_template_path.exists() {
        template_file = match File::open(&exe_template_path) {
            Ok(file) => Some(file),
            Err(_) => None,
        };
    }
    
    // If not found, check all local paths
    if template_file.is_none() {
        for path in &paths_to_check {
            if path.exists() {
                match File::open(path) {
                    Ok(file) => {
                        template_file = Some(file);
                        break;
                    },
                    Err(_) => continue,
                }
            }
        }
    }
    
    // Return error if no template found
    let mut template_file = match template_file {
        Some(file) => file,
        None => {
            eprintln!("Searched in:");
            eprintln!("  - {:?}", exe_template_path);
            for path in paths_to_check {
                eprintln!("  - {:?}", path);
            }
            return Err(anyhow::anyhow!("Template file not found: {}", template_path));
        }
    };
    
    template_file.read_to_string(&mut template_content)?;
    
    // Replace placeholders
    let mut final_content = template_content;
    for (placeholder, value) in replacements {
        let placeholder_tag = format!("{{{{ {} }}}}", placeholder);
        final_content = final_content.replace(&placeholder_tag, value);
    }
    
    // Create parent directories if they don't exist
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write the content to the destination file
    let mut dest_file = File::create(dest_path)
        .with_context(|| format!("Failed to create destination file: {:?}", dest_path))?;
        
    dest_file.write_all(final_content.as_bytes())?;
    
    Ok(())
}
