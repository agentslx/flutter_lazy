use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Result;
use convert_case::{Case, Casing};
use dialoguer;
use console::style;

mod generator;
mod utils;
mod features;
mod swagger;
mod validation;

use generator::FlutterProjectGenerator;
use features::{
    FeatureParams, 
    create_feature, 
    create_auth_feature,
    create_notification_feature, 
    create_main_page_feature
};
use swagger::{SwaggerSource, generate_api_features};

#[derive(Parser)]
#[command(name = "flutter_lazy")]
#[command(about = "Generate a Flutter project with predefined architecture", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a new Flutter project
    New {
        /// Project name
        #[arg(short, long)]
        name: Option<String>,
        
        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Package name (org.example.app)
        #[arg(short, long)]
        package_name: Option<String>,
        
        /// Initialize with a Swagger/OpenAPI spec URL
        #[arg(long = "api-url")]
        api_url: Option<String>,
        
        /// Initialize with a local Swagger/OpenAPI spec file
        #[arg(long = "api-file")]
        api_file: Option<PathBuf>,
        
        /// Skip validation of project structure
        #[arg(long, default_value = "false")]
        no_validate: bool,
    },
    
    /// Validates an existing project or feature structure
    Validate {
        /// Project directory to validate
        #[arg(short, long)]
        project: Option<PathBuf>,
        
        /// Feature name to validate (if validating a specific feature)
        #[arg(short, long)]
        feature: Option<String>,
        
        /// API feature name to validate (if validating a specific API feature)
        #[arg(short, long)]
        api_feature: Option<String>,
    },
    
    /// Creates a new feature in an existing project
    Feature {
        /// Feature name
        #[arg(short, long)]
        name: Option<String>,
        
        /// Project directory
        #[arg(short, long)]
        project: Option<PathBuf>,
        
        /// Skip validation of feature structure
        #[arg(long, default_value = "false")]
        no_validate: bool,
        
        /// Create a minimal feature (UI only, no state management or data layer)
        #[arg(long, default_value = "false")]
        minimal: bool,
        
        /// Skip state management (cubit/bloc) generation
        #[arg(long, default_value = "false")]
        no_state: bool,
        
        /// Skip repository layer generation
        #[arg(long, default_value = "false")]
        no_repository: bool,
        
        /// Skip model classes generation
        #[arg(long, default_value = "false")]
        no_models: bool,
        
        /// Skip UI pages generation
        #[arg(long, default_value = "false")]
        no_pages: bool,
        
        /// Skip services generation
        #[arg(long, default_value = "false")]
        no_services: bool,
        
        /// Skip utilities generation
        #[arg(long, default_value = "false")]
        no_utils: bool,
        
        /// Skip routing configuration
        #[arg(long, default_value = "false")]
        no_routing: bool,
        
        /// Skip dependency injection setup
        #[arg(long, default_value = "false")]
        no_di: bool,
    },
    
    /// Creates a new feature based on a Swagger/OpenAPI specification
    FromApi {
        /// URL to the Swagger/OpenAPI JSON specification
        #[arg(short, long)]
        url: Option<String>,
        
        /// Path to a local Swagger/OpenAPI JSON file
        #[arg(short, long)]
        file: Option<PathBuf>,
        
        /// Project directory
        #[arg(short, long)]
        project: Option<PathBuf>,
        
        /// Only generate specific domains/tags (comma-separated)
        #[arg(short, long)]
        domains: Option<String>,
        
        /// Skip generating cubits/state management (data layer only)
        #[arg(long, default_value = "true")]
        data_only: bool,
        
        /// Skip validation of generated API features
        #[arg(long, default_value = "false")]
        no_validate: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name, output, package_name, api_url, api_file, no_validate } => {
            // Interactive mode if name is not provided
            let project_name = match name {
                Some(n) => n.clone(),
                None => {
                    // Prompt for project name
                    dialoguer::Input::<String>::new()
                        .with_prompt("Enter project name")
                        .validate_with(|input: &String| -> Result<(), String> {
                            if input.is_empty() {
                                Err("Project name cannot be empty".into())
                            } else if input.contains(char::is_whitespace) {
                                Err("Project name cannot contain spaces".into())
                            } else {
                                Ok(())
                            }
                        })
                        .interact()?
                }
            };
            
            // Ask for output directory if not provided
            let output_dir = match output {
                Some(path) => path.clone(),
                None => {
                    let dir_str = dialoguer::Input::<String>::new()
                        .with_prompt("Enter output directory (press Enter for current directory)")
                        .default(".".into())
                        .interact()?;
                    
                    PathBuf::from(dir_str)
                }
            };
            
            // Ask for package name if not provided
            let package = match package_name {
                Some(pkg) => pkg.clone(),
                None => {
                    let snake_case_name = project_name.to_case(Case::Snake);
                    let default_package = format!("com.example.{}", snake_case_name);
                    
                    dialoguer::Input::<String>::new()
                        .with_prompt("Enter package name (e.g., com.example.myapp)")
                        .default(default_package)
                        .validate_with(|input: &String| -> Result<(), String> {
                            if !input.contains('.') {
                                Err("Package name should follow format: com.example.app".into())
                            } else {
                                Ok(())
                            }
                        })
                        .interact()?
                }
            };
            
            println!("\n{}", style("Generating Flutter project...").bold().green());
            let generator = FlutterProjectGenerator::new(&project_name, &Some(output_dir), &Some(package), api_url, api_file)?;
            generator.generate()?;
            
            println!("\n✅ Project {} has been generated successfully!", style(&project_name).bold());
            println!("To start the project:");
            println!("  cd {}", project_name);
            println!("  flutter run --flavor dev -t lib/main_dev.dart");
            
            // Run validation if not explicitly disabled
            if !no_validate {
                println!("\n{}", style("Validating project structure...").bold().cyan());
                let validation_passed = generator.validate()?;
                
                // If validation failed, notify the user
                if !validation_passed {
                    println!("\n{} Some validation checks failed. The project may still work, but might be missing expected files or structure.", 
                        style("⚠️").yellow().bold());
                } else {
                    println!("\n{} Project validation successful!", style("✅").green());
                }
            }
        },
        
        Commands::Feature { 
            name, 
            project, 
            minimal, 
            no_state,
            no_repository,
            no_models,
            no_pages,
            no_services,
            no_utils,
            no_routing,
            no_di,
            no_validate 
        } => {
            println!("{}", style("Feature Generator").bold().cyan());
            
            // Interactive mode if name is not provided
            let feature_name = match name {
                Some(n) => n.clone(),
                None => {
                    // Prepare a list of predefined and common feature types
                    let predefined_features = vec![
                        "Authentication / Auth",
                        "Notifications",
                        "Main Page / Home",
                        "Settings",
                        "Profile",
                        "User Management",
                        "Product List",
                        "Shopping Cart",
                        "Payment",
                        "Chat / Messaging",
                        "Other (custom feature)"
                    ];
                    
                    // Ask user to select feature type
                    let selection = dialoguer::Select::new()
                        .with_prompt("Select feature type")
                        .items(&predefined_features)
                        .default(0)
                        .interact()?;
                    
                    match selection {
                        11 => {
                            // User selected "Other", prompt for custom name
                            dialoguer::Input::<String>::new()
                                .with_prompt("Enter custom feature name")
                                .validate_with(|input: &String| -> Result<(), String> {
                                    if input.is_empty() {
                                        Err("Feature name cannot be empty".into())
                                    } else {
                                        Ok(())
                                    }
                                })
                                .interact()?
                        },
                        _ => {
                            // For predefined features, extract just the feature name 
                            // (e.g., "Authentication / Auth" -> "auth")
                            let feature = &predefined_features[selection];
                            match selection {
                                0 => "auth".to_string(),
                                1 => "notifications".to_string(),
                                2 => "main_page".to_string(),
                                _ => feature.split_whitespace().next().unwrap_or(feature).to_string()
                            }
                        }
                    }
                }
            };
            
            // Determine project directory
            let project_dir = match project {
                Some(path) => path.clone(),
                None => {
                    let dir_str = dialoguer::Input::<String>::new()
                        .with_prompt("Project directory (press Enter for current directory)")
                        .default(".".into())
                        .interact()?;
                    
                    PathBuf::from(dir_str)
                }
            };
            
            // Default to full architecture (not minimal)
            let use_minimal = *minimal;
            
            // Create feature parameters, default to full architecture
            let mut params = if use_minimal {
                FeatureParams::minimal(&feature_name)
            } else {
                FeatureParams::new(&feature_name)
            };
            
            // If not minimal, ask for specific components to include
            if !use_minimal {
                let components = vec![
                    "State Management (BLoC/Cubit)",
                    "Repository Layer",
                    "Data Models",
                    "UI Pages",
                    "Services",
                    "Utils Directory",
                    "Routing Configuration",
                    "Dependency Injection"
                ];
                
                let selections = dialoguer::MultiSelect::new()
                    .with_prompt("Select components to include")
                    .items(&components)
                    .defaults(&[true, true, true, true, true, true, true, true])
                    .interact()?;
                
                // Update params based on selections
                params.has_state_management = selections.contains(&0) && !*no_state;
                params.has_repository = selections.contains(&1) && !*no_repository;
                params.has_models = selections.contains(&2) && !*no_models;
                params.has_pages = selections.contains(&3) && !*no_pages;
                params.has_services = selections.contains(&4) && !*no_services;
                params.has_utils = selections.contains(&5) && !*no_utils;
                params.needs_routing = selections.contains(&6) && !*no_routing;
                params.needs_di = selections.contains(&7) && !*no_di;
            } else {
                // Apply individual flags to override defaults even in minimal mode
                if *no_state {
                    params.has_state_management = false;
                }
                
                if *no_repository {
                    params.has_repository = false;
                }
                
                if *no_models {
                    params.has_models = false;
                }
                
                if *no_pages {
                    params.has_pages = false;
                }
                
                if *no_services {
                    params.has_services = false;
                }
                
                if *no_utils {
                    params.has_utils = false;
                }
                
                if *no_routing {
                    params.needs_routing = false;
                }
                
                if *no_di {
                    params.needs_di = false;
                }
            }
            
            // Create the feature
            println!("\n{}", style("Generating feature...").bold().green());
            
            // Use predefined feature generators for specialized features only
            // These have special implementations beyond just naming conventions
            match feature_name.as_str() {
                "authentication" | "auth" => {
                    create_auth_feature(&project_dir)?;
                },
                "notifications" | "notification" => {
                    create_notification_feature(&project_dir)?;
                },
                "main_page" | "home" => {
                    create_main_page_feature(&project_dir)?;
                },
                _ => {
                    // For all other features (including settings, profile, etc.), use the regular feature creation
                    // This allows adding any new feature dynamically without needing a helper function
                    create_feature(&project_dir, params)?;
                }
            }
            
            println!("\n✅ Feature '{}' created successfully!", style(&feature_name).bold());
            
            // Run validation if not explicitly disabled
            if !no_validate {
                println!("\n{}", style("Validating feature structure...").bold().cyan());
                let validation_rules = validation::ValidationSystem::default_feature_rules(&feature_name);
                let validation_system = validation::ValidationSystem::new(validation_rules);
                let validation_passed = validation_system.run_validation(&project_dir);
                
                if !validation_passed {
                    println!("\n{} Some validation checks failed. The feature may still work, but might be missing expected files or structure.", 
                        style("⚠️").yellow().bold());
                } else {
                    println!("\n{} Feature validation successful!", style("✅").green());
                }
            }
        },
        
        Commands::FromApi { url, file, project, domains, data_only, no_validate } => {
            println!("{}", style("API Feature Generator").bold().cyan());
            
            // Get the Swagger URL or file path
            let source = match (url, file) {
                (Some(u), _) => {
                    println!("Using Swagger API from URL: {}", style(u).bold());
                    SwaggerSource::Url(u.clone())
                },
                (_, Some(f)) => {
                    println!("Using Swagger API from file: {}", style(f.display()).bold());
                    SwaggerSource::File(f.clone())
                },
                _ => {
                    // Interactive mode - prompt for URL
                    let input_url = dialoguer::Input::<String>::new()
                        .with_prompt("Enter Swagger API URL")
                        .interact()?;
                    
                    println!("Using Swagger API from URL: {}", style(&input_url).bold());
                    SwaggerSource::Url(input_url)
                }
            };
            
            // Get the project directory
            let project_dir = match project {
                Some(path) => path.clone(),
                None => {
                    let dir_str = dialoguer::Input::<String>::new()
                        .with_prompt("Enter project directory (press Enter for current directory)")
                        .default(".".into())
                        .interact()?;
                    
                    PathBuf::from(dir_str)
                }
            };
            
            // Filter domains if specified
            let domain_list = domains.as_ref().map(|d| {
                d.split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>()
            });
            
            println!("\n{}", style("Generating API-based features...").bold().green());
            
            // Call the API feature generator
            generate_api_features(&project_dir, source.clone(), domain_list.clone(), *data_only)?;
            
            println!("\n✅ API-based features have been generated successfully!");
            
            // Run validation if not explicitly disabled
            if !no_validate {
                // Validate API features if any domains were specified
                let validation_passed = if let (Some(domains), SwaggerSource::File(_file_path)) = (&domain_list, &source) {
                    println!("\n{}", style("Validating API features...").bold().cyan());
                    
                    let mut all_passed = true;
                    
                    for domain in domains {
                        let feature_name = domain.to_case(Case::Snake);
                        let validation_rules = validation::create_api_feature_validation(&feature_name);
                        let validation_system = validation::ValidationSystem::new(validation_rules);
                        
                        println!("\nValidating API feature: {}", style(&feature_name).bold());
                        let domain_passed = validation_system.run_validation(&project_dir);
                        all_passed = all_passed && domain_passed;
                    }
                    
                    all_passed
                } else if let Some(domains) = &domain_list {
                    // URL source case
                    println!("\n{}", style("Validating API features...").bold().cyan());
                    
                    let mut all_passed = true;
                    
                    for domain in domains {
                        let feature_name = domain.to_case(Case::Snake);
                        let validation_rules = validation::create_api_feature_validation(&feature_name);
                        let validation_system = validation::ValidationSystem::new(validation_rules);
                        
                        println!("\nValidating API feature: {}", style(&feature_name).bold());
                        let domain_passed = validation_system.run_validation(&project_dir);
                        all_passed = all_passed && domain_passed;
                    }
                    
                    all_passed
                } else {
                    // No domains specified, so can't validate specific features
                    println!("\n{}", style("Skipping validation: No specific domains were specified.").yellow());
                    true
                };
                
                if !validation_passed {
                    println!("\n{} Some validation checks failed. The API features may still work, but might be missing expected files or structure.", 
                        style("⚠️").yellow().bold());
                } else {
                    println!("\n{} API feature validation successful!", style("✅").green());
                }
            }
        },
        
        Commands::Validate { project, feature, api_feature } => {
            println!("{}", style("Flutter Lazy Validation").bold().cyan());
            
            // Get project directory
            let project_dir = match project {
                Some(path) => path.clone(),
                None => {
                    let dir_str = dialoguer::Input::<String>::new()
                        .with_prompt("Enter project directory to validate (press Enter for current directory)")
                        .default(".".into())
                        .interact()?;
                    
                    PathBuf::from(dir_str)
                }
            };
            
            // Check if project directory exists and is a Flutter project
            if !project_dir.exists() || !project_dir.is_dir() {
                println!("\n{} Directory does not exist: {}", 
                    style("❌").red(), project_dir.display());
                return Ok(());
            }
            
            if !project_dir.join("pubspec.yaml").exists() {
                println!("\n{} Not a Flutter project (pubspec.yaml not found): {}", 
                    style("❌").red(), project_dir.display());
                return Ok(());
            }
            
            if let Some(feat_name) = feature {
                // Validate a specific feature
                println!("\n{}", style(format!("Validating feature: {}", feat_name)).bold().green());
                
                let validation_rules = validation::ValidationSystem::default_feature_rules(&feat_name);
                let validation_system = validation::ValidationSystem::new(validation_rules);
                validation_system.run_validation(&project_dir);
            } else if let Some(api_feat_name) = api_feature {
                // Validate a specific API feature
                println!("\n{}", style(format!("Validating API feature: {}", api_feat_name)).bold().green());
                
                let validation_rules = validation::create_api_feature_validation(&api_feat_name);
                let validation_system = validation::ValidationSystem::new(validation_rules);
                validation_system.run_validation(&project_dir);
            } else {
                // Validate the entire project
                println!("\n{}", style("Validating project structure...").bold().green());
                
                let validation_rules = validation::ValidationSystem::default_new_project_rules();
                let validation_system = validation::ValidationSystem::new(validation_rules);
                validation_system.run_validation(&project_dir);
            }
        },
    }

    Ok(())
}
