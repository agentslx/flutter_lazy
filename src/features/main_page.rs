// features/main_page.rs
// Main page feature generator

use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use std::fs;
use crate::features::{FeatureParams, create_feature, update_main_router, update_main_di};

/// Create a main page feature with navigation components
pub fn create_main_page_feature(project_dir: &Path) -> Result<()> {
    // Main page feature with bottom navigation,
    // drawer setup, and other navigation elements
    let mut params = FeatureParams::new("main_page");
    
    // Check if the feature already exists
    let feature_dir = project_dir.join("lib/features/main_page");
    if feature_dir.exists() {
        return Err(anyhow::anyhow!("Feature 'main_page' already exists at {:?}", feature_dir));
    }
    
    // Track created files for summary
    let mut created_files = Vec::new();
    
    // Main page uses blocs instead of cubits
    params.has_state_management = false;
    
    println!("Creating main_page feature with navigation components...");
    
    // Standard feature creation
    create_feature(project_dir, params)?;
    
    // Create main_page feature directory
    let feature_dir = project_dir.join("lib/features/main_page");
    
    // Add blocs directory instead of cubits
    fs::create_dir_all(feature_dir.join("blocs"))
        .context("Failed to create blocs directory")?;
    
    // Add widgets directory with navigation-specific components
    fs::create_dir_all(feature_dir.join("ui/widgets"))
        .context("Failed to create widgets directory")?;
    
    // Create main page files using templates
    println!("Creating main page files...");
    
    // Copy template files for UI pages
    let template_pages = [
        ("main_tabs.dart.tmpl", "main_tabs.dart"),
        ("home_page.dart.tmpl", "home_page.dart"),
        ("settings_page.dart.tmpl", "settings_page.dart"),
    ];
    for (template, target) in template_pages.iter() {
        let template_path = PathBuf::from("flutter_lazy/templates/features/main_page/ui/pages").join(template);
        let target_path = feature_dir.join("ui/pages").join(target);
        
        if let Ok(content) = std::fs::read_to_string(&template_path) {
            std::fs::write(&target_path, content)
                .context(format!("Failed to create page file: {}", target))?;
            created_files.push(format!("- UI Page: {}", target_path.display()));
        } else {
            // Fallback if template is not found
            std::fs::write(&target_path, "// TODO: Implement main page\n")
                .context(format!("Failed to create page file: {}", target))?;
            created_files.push(format!("- UI Page: {}", target_path.display()));
        }
    }
    
    // Copy template files for blocs
    let template_blocs = [
        ("bottom_navigation_cubit.dart.tmpl", "bottom_navigation_cubit.dart"),
        ("bottom_navigation_state.dart.tmpl", "bottom_navigation_state.dart"),
    ];
    for (template, target) in template_blocs.iter() {
        let template_path = PathBuf::from("flutter_lazy/templates/features/main_page/blocs/bottom_navigation_cubit").join(template);
        let target_path = feature_dir.join("blocs/bottom_navigation_cubit").join(target);
        
        // Create directory if it doesn't exist
        std::fs::create_dir_all(target_path.parent().unwrap())
            .context("Failed to create blocs directory")?;
            
        if let Ok(content) = std::fs::read_to_string(&template_path) {
            std::fs::write(&target_path, content)
                .context(format!("Failed to create bloc file: {}", target))?;
            created_files.push(format!("- State Management: {}", target_path.display()));
        } else {
            // Fallback if template is not found
            std::fs::write(&target_path, "// TODO: Implement navigation state management\n")
                .context(format!("Failed to create bloc file: {}", target))?;
            created_files.push(format!("- State Management: {}", target_path.display()));
        }
    }
    
    // Copy router file
    let router_template = PathBuf::from("flutter_lazy/templates/features/main_page/router.dart.tmpl");
    let router_path = feature_dir.join("router.dart");
    if let Ok(content) = std::fs::read_to_string(&router_template) {
        std::fs::write(&router_path, content)
            .context("Failed to create router file")?;
        created_files.push(format!("- Router: {}", router_path.display()));
    } else {
        // Fallback if template is not found
        std::fs::write(&router_path, "// TODO: Implement main page router\n")
            .context("Failed to create router file")?;
        created_files.push(format!("- Router: {}", router_path.display()));
    }
    
    // Copy DI file
    let di_template = PathBuf::from("flutter_lazy/templates/features/main_page/di.dart.tmpl");
    let di_path = feature_dir.join("di.dart");
    if let Ok(content) = std::fs::read_to_string(&di_template) {
        std::fs::write(&di_path, content)
            .context("Failed to create DI file")?;
        created_files.push(format!("- DI: {}", di_path.display()));
    } else {
        // Fallback if template is not found
        std::fs::write(&di_path, "// TODO: Implement main page DI\n")
            .context("Failed to create DI file")?;
        created_files.push(format!("- DI: {}", di_path.display()));
    }
    
    // We no longer need custom assets since we're using Material icons
    
    // Update main router file to import this feature's router
    update_main_router(project_dir, "main_page", "MainPage")?;
    
    // Update main DI file to import this feature's DI
    update_main_di(project_dir, "main_page")?;
    
    // Display summary of created files
    println!("\n✅ Main Page feature created successfully with the following components:");
    for file in &created_files {
        println!("{}", file);
    }
    
    Ok(())
}


