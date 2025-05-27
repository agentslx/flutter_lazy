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
    },
    
    /// Creates a new feature in an existing project
    Feature {
        /// Feature name
        #[arg(short, long)]
        name: Option<String>,
        
        /// Project directory
        #[arg(short, long)]
        project: Option<PathBuf>,
        
        /// Whether to create a minimal feature without state management and repositories
        #[arg(short, long, default_value = "false")]
        minimal: bool,
        
        /// Skip state management files (cubit/bloc)
        #[arg(long, default_value = "false")]
        no_state: bool,
        
        /// Skip repository files
        #[arg(long, default_value = "false")]
        no_repository: bool,
        
        /// Skip model files
        #[arg(long, default_value = "false")]
        no_models: bool,
        
        /// Skip UI pages
        #[arg(long, default_value = "false")]
        no_pages: bool,
        
        /// Skip services
        #[arg(long, default_value = "false")]
        no_services: bool,
        
        /// Skip utils directory
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
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name, output, package_name, api_url, api_file } => {
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
            no_di 
        } => {
            println!("{}", style("Feature Generator").bold().cyan());
            
            // Interactive mode if name is not provided
            let feature_name = match name {
                Some(n) => n.clone(),
                None => {
                    // Prepare a list of predefined and common feature types
                    let predefined_features = vec![
                        "auth (specialized)", 
                        "notifications (specialized)", 
                        "main_page (specialized)",
                        "settings", 
                        "profile",
                        "payments",
                        "onboarding",
                        "custom (enter name)"
                    ];
                    
                    // First ask if user wants to use a predefined feature
                    let selection = dialoguer::Select::new()
                        .with_prompt("Select feature type")
                        .items(&predefined_features)
                        .default(7) // Default to custom
                        .interact()?;
                    
                    if selection == 7 { // Custom feature
                        dialoguer::Input::<String>::new()
                            .with_prompt("Enter feature name (e.g., auth, settings, profile)")
                            .validate_with(|input: &String| -> Result<(), String> {
                                if input.is_empty() {
                                    Err("Feature name cannot be empty".into())
                                } else if input.contains(char::is_whitespace) {
                                    Err("Feature name cannot contain spaces".into())
                                } else {
                                    Ok(())
                                }
                            })
                            .interact()?
                    } else {
                        // Extract just the base name without the "(specialized)" text
                        let feature = predefined_features[selection];
                        match selection {
                            0 => "auth".to_string(),
                            1 => "notifications".to_string(),
                            2 => "main_page".to_string(),
                            _ => feature.split_whitespace().next().unwrap_or(feature).to_string()
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
        },
        
        Commands::FromApi { url, file, project, domains, data_only } => {
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
            generate_api_features(&project_dir, source, domain_list, *data_only)?;
            
            println!("\n✅ API-based features have been generated successfully!");
        },
    }

    Ok(())
}
