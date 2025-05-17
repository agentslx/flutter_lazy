// features/auth.rs
// Auth feature generator

use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use std::fs;
use crate::features::{FeatureParams, create_feature, update_main_router, update_main_di};

/// Create an authentication feature with all required components
pub fn create_auth_feature(project_dir: &Path) -> Result<()> {
    // Authentication feature has a specialized implementation with multiple cubits
    let params = FeatureParams::new("auth");
    
    // Check if the feature already exists
    let feature_dir = project_dir.join("lib/features/auth");
    if feature_dir.exists() {
        return Err(anyhow::anyhow!("Feature 'auth' already exists at {:?}", feature_dir));
    }
    
    println!("Creating auth feature with specialized components...");
    
    // Standard feature creation first (this will create the base directory structure)
    create_feature(project_dir, params)?;
    
    // Track created files for summary
    let mut created_files = Vec::new();
    
    // Create auth feature directory
    let feature_dir = project_dir.join("lib/features/auth");
    
    // Create specialized auth cubits directories to match the existing structure
    let auth_cubits = [
        "login_cubit",
        "register_cubit",
        "forgot_password_cubit",
        "verify_email_cubit",
        "create_password_cubit",
        "personal_details_cubit",
        "welcome_cubit",
        "loading_cubit"
    ];
    
    // Create auth pages matching the existing structure
    let auth_pages = [
        "login_page.dart",
        "register_email_password_page.dart",
        "register_personal_details_page.dart",
        "forgot_password_page.dart",
        "verify_email_page.dart",
        "create_password_page.dart",
        "welcome_page.dart",
        "loading_page.dart"
    ];
    
    println!("Creating auth specific cubits...");
    
    // Create directories for specialized cubits
    for cubit in auth_cubits.iter() {
        fs::create_dir_all(feature_dir.join("cubits").join(cubit))
            .context(format!("Failed to create {} directory", cubit))?;
            
        // Here we would create the specialized cubit files for each auth cubit
        // copy_template_file(...) for each cubit
    }
    
    // Add services directory which is specific to auth feature
    fs::create_dir_all(feature_dir.join("services"))
        .context("Failed to create services directory")?;
    
    // Create auth UI pages
    println!("Creating auth pages...");
    for page in auth_pages.iter() {
        // We would create or copy templates for each page
        // For now, just create empty files to ensure directory structure
        let page_path = feature_dir.join("ui/pages").join(page);
        std::fs::write(&page_path, "// TODO: Implement auth page\n")
            .context(format!("Failed to create page file: {}", page))?;
        created_files.push(format!("- UI Page: {}", page_path.display()));
    }
    
    // Create auth-specific models and repositories
    fs::create_dir_all(feature_dir.join("data/models"))
        .context("Failed to create auth models directory")?;
    
    fs::create_dir_all(feature_dir.join("data/repository"))
        .context("Failed to create auth repository directory")?;
    
    // Create basic auth models
    let model_path = feature_dir.join("data/models/auth_model.dart");
    std::fs::write(&model_path, 
        "class AuthModel {}\n\nclass UserModel {}\n")
        .context("Failed to create auth model")?;
    created_files.push(format!("- Model: {}", model_path.display()));
    
    // Create basic auth repository
    let repo_path = feature_dir.join("data/repository/auth_repository.dart");
    std::fs::write(&repo_path, 
        "abstract class AuthRepository {}\n\nclass AuthRepositoryImpl implements AuthRepository {}\n")
        .context("Failed to create auth repository")?;
    created_files.push(format!("- Repository: {}", repo_path.display()));
    
    // Update main router file to import this feature's router
    update_main_router(project_dir, "auth", "Auth")?;
    
    // Update main DI file to import this feature's DI
    update_main_di(project_dir, "auth")?;
    
    // Display summary of created files
    println!("\n✅ Auth feature created successfully with the following components:");
    for file in &created_files {
        println!("{}", file);
    }
    
    Ok(())
}
