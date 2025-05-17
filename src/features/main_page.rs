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
    
    // Create main page files
    println!("Creating main page files...");
    
    // Create UI pages
    let main_pages = ["main_page.dart", "navigation_shell.dart"];
    for page in main_pages.iter() {
        let page_path = feature_dir.join("ui/pages").join(page);
        std::fs::write(&page_path, "// TODO: Implement main page\n")
            .context(format!("Failed to create page file: {}", page))?;
        created_files.push(format!("- UI Page: {}", page_path.display()));
    }
    
    // Create navigation widgets
    let widgets = ["bottom_navigation_bar.dart", "drawer_menu.dart"];
    for widget in widgets.iter() {
        let widget_path = feature_dir.join("ui/widgets").join(widget);
        std::fs::write(&widget_path, "// TODO: Implement navigation widget\n")
            .context(format!("Failed to create widget file: {}", widget))?;
        created_files.push(format!("- UI Widget: {}", widget_path.display()));
    }
    
    // Create bloc files
    let bloc_files = ["navigation_bloc.dart", "navigation_event.dart", "navigation_state.dart"];
    for file in bloc_files.iter() {
        let bloc_path = feature_dir.join("blocs").join(file);
        std::fs::write(&bloc_path, "// TODO: Implement navigation bloc\n")
            .context(format!("Failed to create bloc file: {}", file))?;
        created_files.push(format!("- State Management: {}", bloc_path.display()));
    }
    
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
