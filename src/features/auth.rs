// features/auth.rs
// Auth feature generator

use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use std::fs;
use crate::features::{FeatureParams, create_feature, update_main_router, update_main_di};
use crate::utils::copy_template_file;

/// Create an authentication feature with all required components
pub fn create_auth_feature(project_dir: &Path) -> Result<()> {
    // Authentication feature has a specialized implementation with multiple cubits
    // Use minimal params to avoid generating the generic auth_cubit
    let params = FeatureParams::minimal("auth");
    
    // Check if the feature already exists
    let feature_dir = project_dir.join("lib/features/auth");
    if feature_dir.exists() {
        return Err(anyhow::anyhow!("Feature 'auth' already exists at {:?}", feature_dir));
    }
    
    println!("Creating auth feature with specialized components...");
    
    // Create the base directory structure but without generic state management
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
    }
    
    // Copy specialized auth cubits and states
    let auth_cubit_templates = [
        // Basic cubits
        ("features/auth/cubits/login_cubit/login_cubit.dart.tmpl", "cubits/login_cubit/login_cubit.dart"),
        ("features/auth/cubits/register_cubit/register_cubit.dart.tmpl", "cubits/register_cubit/register_cubit.dart"),
        ("features/auth/cubits/forgot_password_cubit/forgot_password_cubit.dart.tmpl", "cubits/forgot_password_cubit/forgot_password_cubit.dart"),
        
        // Optional cubits
        ("features/auth/cubits/verify_email_cubit/verify_email_cubit.dart.tmpl", "cubits/verify_email_cubit/verify_email_cubit.dart"),
        ("features/auth/cubits/create_password_cubit/create_password_cubit.dart.tmpl", "cubits/create_password_cubit/create_password_cubit.dart"),
        ("features/auth/cubits/welcome_cubit/welcome_cubit.dart.tmpl", "cubits/welcome_cubit/welcome_cubit.dart"),
        
        // Basic states
        ("features/auth/cubits/login_cubit/login_state.dart.tmpl", "cubits/login_cubit/login_state.dart"),
        ("features/auth/cubits/register_cubit/register_state.dart.tmpl", "cubits/register_cubit/register_state.dart"),
        ("features/auth/cubits/forgot_password_cubit/forgot_password_state.dart.tmpl", "cubits/forgot_password_cubit/forgot_password_state.dart"),
        
        // Optional states
        ("features/auth/cubits/verify_email_cubit/verify_email_state.dart.tmpl", "cubits/verify_email_cubit/verify_email_state.dart"),
        ("features/auth/cubits/create_password_cubit/create_password_state.dart.tmpl", "cubits/create_password_cubit/create_password_state.dart"),
        ("features/auth/cubits/welcome_cubit/welcome_state.dart.tmpl", "cubits/welcome_cubit/welcome_state.dart"),
    ];
    
    for (template, dest_path) in auth_cubit_templates.iter() {
        copy_template_file(
            template,
            &feature_dir.join(dest_path), 
            &[]
        ).context(format!("Failed to copy template {}", template))?;
        
        let file_type = if dest_path.contains("state.dart") { "State" } else { "Cubit" };
        created_files.push(format!("- {}: {}", file_type, feature_dir.join(dest_path).display()));
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
    
    // Copy auth models
    let model_path = feature_dir.join("data/models/user_model.dart");
    copy_template_file(
        "features/auth/data/models/user_model.dart.tmpl",
        &model_path,
        &[]
    ).context("Failed to copy user model template")?;
    created_files.push(format!("- Model: {}", model_path.display()));
    
    // Copy auth repository
    let repo_path = feature_dir.join("data/repository/auth_repository.dart");
    copy_template_file(
        "features/auth/data/repository/auth_repository.dart.tmpl",
        &repo_path,
        &[]
    ).context("Failed to copy auth repository template")?;
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
