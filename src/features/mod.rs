// features/mod.rs
// Module structure for features

pub mod auth;
pub mod main_page;
pub mod notifications;

use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use std::fs;
use crate::utils::copy_template_file;

// Re-export feature functions
pub use auth::create_auth_feature;
pub use main_page::create_main_page_feature;
pub use notifications::create_notification_feature;

/// Parameters for feature generation
pub struct FeatureParams {
    pub name: String,
    pub has_state_management: bool,
    pub has_repository: bool,
    pub has_models: bool,
    pub has_pages: bool,
    pub has_services: bool,
    pub has_utils: bool,
    pub needs_routing: bool,
    pub needs_di: bool,
}

impl FeatureParams {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            has_state_management: true,
            has_repository: true,
            has_models: true,
            has_pages: true,
            has_services: true,
            has_utils: true,
            needs_routing: true,
            needs_di: true,
        }
    }
    
    pub fn minimal(name: &str) -> Self {
        Self {
            name: name.to_string(),
            has_state_management: false,
            has_repository: false,
            has_models: false,
            has_pages: true,
            has_services: false,
            has_utils: false,
            needs_routing: true,
            needs_di: false,
        }
    }
    
    pub fn ui_only(name: &str) -> Self {
        Self {
            name: name.to_string(),
            has_state_management: false,
            has_repository: false,
            has_models: false,
            has_pages: true,
            has_services: false,
            has_utils: false,
            needs_routing: true,
            needs_di: true,
        }
    }
    
    pub fn with_state_type(name: &str, state_type: &str) -> Self {
        let mut params = Self::new(name);
        
        // Override state management type based on input
        if state_type == "bloc" {
            params.has_state_management = false; // We'll handle it separately
        }
        
        params
    }
}

/// Create a general feature with the given parameters
pub fn create_feature(project_dir: &Path, params: FeatureParams) -> Result<()> {
    let feature_name = params.name.to_case(Case::Snake);
    let feature_dir = project_dir.join("lib/features").join(&feature_name);
    
    // Check if the feature already exists
    if feature_dir.exists() {
        return Err(anyhow::anyhow!("Feature '{}' already exists at {:?}", feature_name, feature_dir));
    }
    
    println!("Creating feature: {}", feature_name);
    
    // Create base directory
    fs::create_dir_all(&feature_dir)
        .with_context(|| format!("Failed to create feature directory: {:?}", feature_dir))?;
    
    // Create subdirectories based on feature parameters
    if params.has_state_management {
        fs::create_dir_all(feature_dir.join("cubits"))
            .context("Failed to create cubits directory")?;
    }
    
    if params.has_repository {
        fs::create_dir_all(feature_dir.join("data/repository"))
            .context("Failed to create repository directory")?;
        
        if params.has_models {
            fs::create_dir_all(feature_dir.join("data/models"))
                .context("Failed to create models directory")?;
        }
        
        fs::create_dir_all(feature_dir.join("data/datasources"))
            .context("Failed to create datasources directory")?;
    }
    
    if params.has_pages {
        fs::create_dir_all(feature_dir.join("ui/pages"))
            .context("Failed to create pages directory")?;
        fs::create_dir_all(feature_dir.join("ui/_widgets"))
            .context("Failed to create _widgets directory")?;
    }
    
    // Create services directory if needed
    if params.has_services {
        fs::create_dir_all(feature_dir.join("services"))
            .context("Failed to create services directory")?;
    }
    
    // Create utils directory if needed
    if params.has_utils {
        fs::create_dir_all(feature_dir.join("utils"))
            .context("Failed to create utils directory")?;
    }
    
    // Create template files with replacements
    let pascal_name = params.name.to_case(Case::Pascal);
    let snake_name = params.name.to_case(Case::Snake);
    let camel_name = params.name.to_case(Case::Camel);
    
    // Generate basic template files
    generate_feature_files(
        &feature_dir, 
        &pascal_name,
        &snake_name,
        &camel_name,
        &params
    )?;
    
    Ok(())
}

// Generate feature files from templates
fn generate_feature_files(
    feature_dir: &Path, 
    pascal_name: &str,
    snake_name: &str,
    camel_name: &str,
    params: &FeatureParams
) -> Result<()> {
    // Common replacements for all templates
    let replacements = [
        ("FEATURE_NAME_PASCAL", pascal_name),
        ("FEATURE_NAME_SNAKE", snake_name),
        ("FEATURE_NAME_CAMEL", camel_name),
    ];
    
    // Track created files for summary
    let mut created_files = Vec::new();
    
    // Generate state management files if needed
    if params.has_state_management {
        // Create cubit directory
        fs::create_dir_all(feature_dir.join("cubits").join(format!("{}_cubit", snake_name)))
            .context("Failed to create cubit directory")?;
        
        // Create cubit files with simplified boilerplate
        let cubit_file = feature_dir.join("cubits").join(format!("{}_cubit", snake_name)).join(format!("{}_cubit.dart", snake_name));
        copy_template_file(
            "features/common/cubits/feature_cubit/feature_cubit.dart.tmpl",
            &cubit_file,
            &replacements
        )?;
        created_files.push(format!("- State Management: {}", cubit_file.display()));
        
        // Create state file
        let state_file = feature_dir.join("cubits").join(format!("{}_cubit", snake_name)).join(format!("{}_state.dart", snake_name));
        copy_template_file(
            "features/common/cubits/feature_cubit/feature_state.dart.tmpl",
            &state_file,
            &replacements
        )?;
        created_files.push(format!("- State: {}", state_file.display()));
    }
    
    // Generate repository if needed
    if params.has_repository {
        let repo_file = feature_dir.join("data/repository").join(format!("{}_repository.dart", snake_name));
        copy_template_file(
            "features/common/data/repository/feature_repository.dart.tmpl",
            &repo_file,
            &replacements
        )?;
        created_files.push(format!("- Repository: {}", repo_file.display()));
        
        // Create model if needed
        if params.has_models {
            let model_file = feature_dir.join("data/models").join(format!("{}_model.dart", snake_name));
            copy_template_file(
                "features/common/data/models/feature_model.dart.tmpl",
                &model_file,
                &replacements
            )?;
            created_files.push(format!("- Model: {}", model_file.display()));
        }
    }
    
    // Generate UI pages if needed
    if params.has_pages {
        // Create page
        let page_file = feature_dir.join("ui/pages").join(format!("{}_page.dart", snake_name));
        copy_template_file(
            "features/common/ui/pages/feature_page.dart.tmpl",
            &page_file,
            &replacements
        )?;
        created_files.push(format!("- UI Page: {}", page_file.display()));
        
        // Create widget
        let widget_file = feature_dir.join("ui/_widgets").join(format!("{}_item_widget.dart", snake_name));
        copy_template_file(
            "features/common/ui/_widgets/feature_item_widget.dart.tmpl",
            &widget_file,
            &replacements
        )?;
        created_files.push(format!("- UI Widget: {}", widget_file.display()));
    }
    
    // Generate router if needed
    if params.needs_routing {
        let router_file = feature_dir.join("router.dart");
        copy_template_file(
            "features/common/router.dart.tmpl",
            &router_file,
            &replacements
        )?;
        created_files.push(format!("- Router: {}", router_file.display()));
        
        // Update main router file to import this feature's router
        let project_dir_path = feature_dir.parent().and_then(|f| f.parent()).and_then(|f| f.parent()).unwrap_or(Path::new("."));
        update_main_router(project_dir_path, snake_name, pascal_name)?;
    }
    
    // Generate services if needed
    if params.has_services {
        let service_file = feature_dir.join("services").join(format!("{}_service.dart", snake_name));
        copy_template_file(
            "features/common/services/feature_service.dart.tmpl",
            &service_file,
            &replacements
        )?;
        created_files.push(format!("- Service: {}", service_file.display()));
    }
    
    // Generate utils if needed
    if params.has_utils {
        let utils_file = feature_dir.join("utils").join(format!("{}_helpers.dart", snake_name));
        copy_template_file(
            "features/common/utils/feature_helpers.dart.tmpl",
            &utils_file,
            &replacements
        )?;
        created_files.push(format!("- Utils: {}", utils_file.display()));
    }

    // Generate DI if needed
    if params.needs_di {
        let di_file = feature_dir.join("di.dart");
        copy_template_file(
            "features/common/di.dart.tmpl",
            &di_file,
            &replacements
        )?;
        created_files.push(format!("- DI: {}", di_file.display()));
        
        // Update main DI file to import this feature's DI
        let project_dir_path = feature_dir.parent().and_then(|f| f.parent()).and_then(|f| f.parent()).unwrap_or(Path::new("."));
        update_main_di(project_dir_path, snake_name)?;
    }
    
    // Display summary of created files
    println!("\n✅ Feature created successfully with the following components:");
    for file in &created_files {
        println!("{}", file);
    }
    
    println!("\nℹ️  Using simplified templates with reduced boilerplate");
    
    Ok(())
}

// Update the main router file to include the new feature's routes
pub fn update_main_router(project_dir: &Path, feature_name: &str, pascal_name: &str) -> Result<()> {
    let router_file_path = project_dir.join("lib/router.dart");
    
    if !router_file_path.exists() {
        println!("Main router.dart not found at {:?}, skipping router integration", router_file_path);
        return Ok(());
    }
    
    let mut content = std::fs::read_to_string(&router_file_path)
        .context("Failed to read router.dart")?;
    
    // Add import if not already present
    let import_line = format!("import 'features/{}/router.dart';", feature_name);
    if !content.contains(&import_line) {
        // Find the last import line
        if let Some(pos) = content.rfind("import ") {
            if let Some(end) = content[pos..].find(';') {
                let insert_pos = pos + end + 1;
                content.insert_str(insert_pos, &format!("\n{}", import_line));
            }
        }
    }
    
    // Add route registration
    // Look for routes list or GoRouter configuration
    if let Some(routes_pos) = content.find("routes: [") {
        // Find position after the opening bracket
        let insert_pos = routes_pos + "routes: [".len();
        
        // Add the new route
        let route_line = format!("\n      // {} Routes\n      ...{}Routes,", pascal_name, feature_name);
        content.insert_str(insert_pos, &route_line);
    } else {
        println!("Could not locate routes list in router.dart, please manually add {} routes", pascal_name);
    }
    
    // Write the updated content back to the file
    std::fs::write(&router_file_path, content)
        .context("Failed to write updated router.dart")?;
    
    println!("✅ Updated main router.dart with {} routes", pascal_name);
    
    Ok(())
}

// Update the main DI file to include the new feature's dependencies
pub fn update_main_di(project_dir: &Path, feature_name: &str) -> Result<()> {
    let di_file_path = project_dir.join("lib/di.dart");
    
    if !di_file_path.exists() {
        println!("Main di.dart not found at {:?}, skipping DI integration", di_file_path);
        return Ok(());
    }
    
    let mut content = std::fs::read_to_string(&di_file_path)
        .context("Failed to read di.dart")?;
    
    // Add import if not already present
    let import_line = format!("import 'features/{}/di.dart';", feature_name);
    if !content.contains(&import_line) {
        // Find the last import line
        if let Some(pos) = content.rfind("import ") {
            if let Some(end) = content[pos..].find(';') {
                let insert_pos = pos + end + 1;
                content.insert_str(insert_pos, &format!("\n{}", import_line));
            }
        }
    }
    
    // Add DI registration
    if let Some(setup_pos) = content.find("void setupDependencyInjection()") {
        // Find the function body opening brace
        if let Some(body_start) = content[setup_pos..].find('{') {
            let insert_pos = setup_pos + body_start + 1;
            
            // Add the new DI setup call
            let di_line = format!("\n  // Register {} dependencies\n  register{}Dependencies();", 
                feature_name.replace("_", " ").to_string(), 
                feature_name.split('_')
                    .map(|word| word.chars().next().unwrap_or_default().to_uppercase().to_string() + &word[1..])
                    .collect::<String>());
            
            content.insert_str(insert_pos, &di_line);
        }
    } else {
        println!("Could not locate setupDependencyInjection function in di.dart, please manually add {} dependencies", feature_name);
    }
    
    // Write the updated content back to the file
    std::fs::write(&di_file_path, content)
        .context("Failed to write updated di.dart")?;
    
    println!("✅ Updated main di.dart with {} dependencies", feature_name);
    
    Ok(())
}

// Welcome back feature has been removed
