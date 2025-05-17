// features/notifications.rs
// Notifications feature generator

use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use std::fs;
use crate::features::{FeatureParams, create_feature, update_main_router, update_main_di};

/// Create a notifications feature with required components
pub fn create_notification_feature(project_dir: &Path) -> Result<()> {
    // Notification feature has specialized implementation for handling
    // push notifications, local notifications, etc.
    let params = FeatureParams::new("notifications");
    
    // Check if the feature already exists
    let feature_dir = project_dir.join("lib/features/notifications");
    if feature_dir.exists() {
        return Err(anyhow::anyhow!("Feature 'notifications' already exists at {:?}", feature_dir));
    }
    
    // Track created files for summary
    let mut created_files = Vec::new();
    
    println!("Creating notifications feature with specialized components...");
    
    // Standard feature creation
    create_feature(project_dir, params)?;
    
    // Create notifications feature directory
    let feature_dir = project_dir.join("lib/features/notifications");
    
    // Create notification services directory
    fs::create_dir_all(feature_dir.join("services"))
        .context("Failed to create notification services directory")?;
    
    // Create notification service files
    println!("Creating notification service files...");
    let service_files = [
        "notification_service.dart",
        "local_notification_service.dart",
        "push_notification_service.dart"
    ];
    
    for file in service_files.iter() {
        let service_path = feature_dir.join("services").join(file);
        std::fs::write(&service_path, "// TODO: Implement notification service\n")
            .context(format!("Failed to create service file: {}", file))?;
        created_files.push(format!("- Service: {}", service_path.display()));
    }
    
    // Create UI components for notifications
    let pages = ["notification_page.dart", "notification_details_page.dart"];
    for page in pages.iter() {
        let page_path = feature_dir.join("ui/pages").join(page);
        std::fs::write(&page_path, "// TODO: Implement notification page\n")
            .context(format!("Failed to create page file: {}", page))?;
        created_files.push(format!("- UI Page: {}", page_path.display()));
    }
    
    // Create notification model
    fs::create_dir_all(feature_dir.join("data/models"))
        .context("Failed to create notification models directory")?;
    
    let model_path = feature_dir.join("data/models/notification_model.dart");
    std::fs::write(&model_path, 
        "class NotificationModel {}\n")
        .context("Failed to create notification model")?;
    created_files.push(format!("- Model: {}", model_path.display()));
    
    // Update main router file to import this feature's router
    update_main_router(project_dir, "notifications", "Notifications")?;
    
    // Update main DI file to import this feature's DI
    update_main_di(project_dir, "notifications")?;
    
    // Display summary of created files
    println!("\n✅ Notification feature created successfully with the following components:");
    for file in &created_files {
        println!("{}", file);
    }
    
    Ok(())
}
