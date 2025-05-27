use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use anyhow::{Context, Result};
use dialoguer::{MultiSelect, Input, Select};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use convert_case::{Case, Casing};
use fs_extra::dir::CopyOptions;

use crate::utils::copy_template_file;
use crate::features::{create_auth_feature, create_notification_feature, create_main_page_feature};
use crate::swagger;

pub struct ProjectConfig {
    pub name: String,
    pub package_name: String,
    pub flavors: Vec<String>,
    pub features: Vec<String>,
    pub output_dir: PathBuf,
}

// API specification structure to hold Swagger source and domain filter
pub struct ApiSpec {
    pub source: swagger::SwaggerSource,
    pub domains: Option<Vec<String>>,
}

pub struct FlutterProjectGenerator {
    config: ProjectConfig,
    api_spec: Option<ApiSpec>,
}

impl FlutterProjectGenerator {
    pub fn new(name: &str, output: &Option<PathBuf>, package_name: &Option<String>,
              api_url: &Option<String>, api_file: &Option<PathBuf>) -> Result<Self> {
        let output_dir = output.clone().unwrap_or_else(|| PathBuf::from("."));
        
        // Convert project name to snake_case for directory
        let project_dir_name = name.to_case(Case::Snake);
        
        // Determine package name: use provided or generate from project name
        let default_package_name = format!("com.example.{}", project_dir_name);
        let package_name = package_name.clone().unwrap_or(default_package_name);
        
        // Ask for flavors
        println!("{}", style("Project Setup").bold().cyan());
        let default_flavors = vec!["dev", "stage", "prod"];
        let flavors = MultiSelect::new()
            .with_prompt("Select flavors (Space to select, Enter to confirm)")
            .items(&default_flavors)
            .defaults(&[true, true, true])
            .interact()?
            .iter()
            .map(|&i| default_flavors[i].to_string())
            .collect();
        
        // Ask for features
        let available_features = vec!["auth", "notifications", "main_page"];
        let selected_features = MultiSelect::new()
            .with_prompt("Select features to include")
            .items(&available_features)
            .defaults(&[true, true, true, false])
            .interact()?
            .iter()
            .map(|&i| available_features[i].to_string())
            .collect();

        // Process API specification if provided
        let api_spec = Self::process_api_spec(api_url, api_file)?;
        
        Ok(Self {
            config: ProjectConfig {
                name: name.to_string(),
                package_name,
                flavors,
                features: selected_features,
                output_dir: output_dir.join(project_dir_name),
            },
            api_spec,
        })
    }
    
    /// Process API specification from URL or file
    fn process_api_spec(
        api_url: &Option<String>, 
        api_file: &Option<PathBuf>
    ) -> Result<Option<ApiSpec>> {
        // If no API source is provided, ask if user wants to include an API spec
        if api_url.is_none() && api_file.is_none() {
            let include_api = Select::new()
                .with_prompt("Would you like to generate features from a Swagger/OpenAPI specification?")
                .items(&["No", "Yes, from URL", "Yes, from local file"])
                .default(0)
                .interact()?;
            
            if include_api == 0 {
                // User chose not to include an API spec
                return Ok(None);
            }
            
            if include_api == 1 {
                // User chose to include an API spec from URL
                let url = Self::prompt_for_api_url(None)?;
                return Self::create_api_spec(Some(url), None);
            } else {
                // User chose to include an API spec from file
                let file_path = Self::prompt_for_api_file(None)?;
                return Self::create_api_spec(None, Some(file_path));
            }
        }
        
        // Process provided API URL or file
        Self::create_api_spec(api_url.clone(), api_file.clone())
    }

    /// Create API spec from URL or file with proper error handling and retry
    fn create_api_spec(
        api_url: Option<String>,
        api_file: Option<PathBuf>
    ) -> Result<Option<ApiSpec>> {
        // Process API URL if provided
        if let Some(url) = api_url {
            println!("Loading API specification from URL: {}", style(&url).bold());
            
            // Attempt to load the Swagger spec
            match Self::try_load_swagger_from_url(&url) {
                Ok(source) => {
                    // Ask for domain filter
                    let domains = Self::prompt_for_domains()?;
                    
                    return Ok(Some(ApiSpec {
                        source,
                        domains,
                    }));
                },
                Err(err) => {
                    // Handle error and offer retry
                    println!("{}: {}", style("Error loading API specification").red(), err);
                    let retry = Select::new()
                        .with_prompt("What would you like to do?")
                        .items(&["Try a different URL", "Try a local file instead", "Skip API specification"])
                        .default(0)
                        .interact()?;
                    
                    match retry {
                        0 => { // Try different URL
                            let new_url = Self::prompt_for_api_url(None)?;
                            return Self::create_api_spec(Some(new_url), None);
                        },
                        1 => { // Try local file
                            let file_path = Self::prompt_for_api_file(None)?;
                            return Self::create_api_spec(None, Some(file_path));
                        },
                        _ => { // Skip
                            return Ok(None);
                        }
                    }
                }
            }
        }
        
        // Process API file if provided
        if let Some(path) = api_file {
            println!("Loading API specification from file: {}", style(path.display()).bold());
            
            // Attempt to load the Swagger spec
            match Self::try_load_swagger_from_file(&path) {
                Ok(source) => {
                    // Ask for domain filter
                    let domains = Self::prompt_for_domains()?;
                    
                    return Ok(Some(ApiSpec {
                        source,
                        domains,
                    }));
                },
                Err(err) => {
                    // Handle error and offer retry
                    println!("{}: {}", style("Error loading API specification").red(), err);
                    let retry = Select::new()
                        .with_prompt("What would you like to do?")
                        .items(&["Try a different file", "Try a URL instead", "Skip API specification"])
                        .default(0)
                        .interact()?;
                    
                    match retry {
                        0 => { // Try different file
                            let new_path = Self::prompt_for_api_file(None)?;
                            return Self::create_api_spec(None, Some(new_path));
                        },
                        1 => { // Try URL
                            let url = Self::prompt_for_api_url(None)?;
                            return Self::create_api_spec(Some(url), None);
                        },
                        _ => { // Skip
                            return Ok(None);
                        }
                    }
                }
            }
        }
        
        // No API source provided
        Ok(None)
    }

    /// Prompt user for API URL with validation
    fn prompt_for_api_url(default_url: Option<&str>) -> Result<String> {
        let default = default_url.unwrap_or("https://petstore.swagger.io/v2/swagger.json");
        
        let url = Input::<String>::new()
            .with_prompt("Enter the Swagger/OpenAPI URL")
            .default(default.into())
            .validate_with(|input: &String| -> Result<(), String> {
                if !input.starts_with("http://") && !input.starts_with("https://") {
                    return Err("URL must start with http:// or https://".into());
                }
                Ok(())
            })
            .interact()?;
        
        Ok(url)
    }

    /// Prompt user for API file path with validation
    fn prompt_for_api_file(default_path: Option<&PathBuf>) -> Result<PathBuf> {
        let default_str = match default_path {
            Some(p) => p.to_string_lossy().to_string(),
            None => String::from("./api-spec.json"),
        };
        
        let path_str = Input::<String>::new()
            .with_prompt("Enter the Swagger/OpenAPI file path")
            .default(default_str)
            .validate_with(|input: &String| -> Result<(), String> {
                let path = PathBuf::from(input);
                if !path.exists() {
                    return Err(format!("File does not exist: {}", input));
                }
                Ok(())
            })
            .interact()?;
        
        Ok(PathBuf::from(path_str))
    }

    /// Prompt user for domains to include (optional)
    fn prompt_for_domains() -> Result<Option<Vec<String>>> {
        let include_filter = Select::new()
            .with_prompt("Would you like to filter API domains/tags?")
            .items(&["No, include all domains", "Yes, specify domains to include"])
            .default(0)
            .interact()?;
        
        if include_filter == 0 {
            return Ok(None);
        }
        
        let domains_str = Input::<String>::new()
            .with_prompt("Enter domain names to include (comma-separated)")
            .interact()?;
        
        let domains = domains_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>();
        
        if domains.is_empty() {
            return Ok(None);
        }
        
        Ok(Some(domains))
    }

    /// Try to load a Swagger spec from URL with error handling
    fn try_load_swagger_from_url(url: &str) -> Result<swagger::SwaggerSource> {
        Ok(swagger::SwaggerSource::Url(url.to_string()))
    }

    /// Try to load a Swagger spec from file with error handling
    fn try_load_swagger_from_file(path: &PathBuf) -> Result<swagger::SwaggerSource> {
        if !path.exists() {
            return Err(anyhow::anyhow!("File does not exist: {:?}", path));
        }
        Ok(swagger::SwaggerSource::File(path.clone()))
    }

    pub fn generate(&self) -> Result<()> {
        self.create_base_project()?;
        self.setup_project_structure()?;
        self.create_flavors()?;
        self.add_features()?;
        self.update_pubspec()?;
        self.process_api_features()?;
        
        Ok(())
    }
    
    /// Process API features if API spec is provided
    fn process_api_features(&self) -> Result<()> {
        if let Some(api_spec) = &self.api_spec {
            println!("{}", style("Generating API features...").bold().green());
            
            swagger::generate_api_features(
                &self.config.output_dir,
                api_spec.source.clone(),
                api_spec.domains.clone(),
                true // data_only by default
            )?;
            
            println!("✅ API features generated");
        }
        
        Ok(())
    }
    
    // ... rest of the implementation ...
    
    fn create_base_project(&self) -> Result<()> {
        println!("Creating base Flutter project...");
        let pb = self.create_progress_bar(3);
        
        // Run flutter create command
        let output = Command::new("flutter")
            .args(&[
                "create",
                "--org", &self.config.package_name,
                "--project-name", &self.config.name.to_case(Case::Snake),
                self.config.output_dir.to_str().unwrap(),
            ])
            .output()
            .context("Failed to execute flutter create command")?;
        
        if !output.status.success() {
            return Err(anyhow::anyhow!("Flutter create command failed"));
        }
        pb.inc(1);
        
        // Copy base structure
        self.copy_template_dir("common", &self.config.output_dir)?;
        pb.inc(1);
        
        // Create placeholder assets
        self.setup_assets()?;
        pb.inc(1);
        
        pb.finish_and_clear();
        println!("✅ Base project created");
        Ok(())
    }
    
    fn setup_project_structure(&self) -> Result<()> {
        println!("Setting up project structure...");
        let lib_dir = self.config.output_dir.join("lib");
        
        let directories = [
            "config", 
            "core/entities",
            "core/enums",
            "core/failures",
            "core/form",
            "core/models",
            "core/extensions",
            "core/utils",
            "features", 
            "helpers", 
            "modules/bloc", 
            "modules/local_storage_module", 
            "modules/push_notification", 
            "modules/rest_module",
            "widgets/buttons",
            "widgets/cards",
            "widgets/dialogs",
            "widgets/inputs"
        ];
        
        let pb = self.create_progress_bar(directories.len() as u64);
        
        for dir in directories.iter() {
            std::fs::create_dir_all(lib_dir.join(dir))
                .context(format!("Failed to create directory: {}", dir))?;
            pb.inc(1);
        }
        
        // Create core files and other setup
        // Copy base core files
        let core_files = [
            ("core/failures/failure.dart", "core/failures/failure.dart"),
            ("core/utils/logger.dart", "core/utils/logger.dart"),
        ];
        
        for (src, dest) in core_files.iter() {
            self.copy_template_file(
                src, 
                &self.config.output_dir.join("lib").join(dest),
                &[("{package_name}", &self.config.package_name)]
            )?;
        }
        
        pb.finish_and_clear();
        println!("✅ Directory structure created");
        Ok(())
    }
    
    fn create_flavors(&self) -> Result<()> {
        println!("Setting up flavors...");
        let pb = self.create_progress_bar(self.config.flavors.len() as u64 + 1);
        
        // Create main.dart that uses flavors
        self.copy_template_file(
            "common/main_flavor.dart.tmpl",
            &self.config.output_dir.join("lib/main.dart"),
            &[("{package_name}", &self.config.package_name)]
        )?;
        pb.inc(1);
        
        // Create a main file for each flavor
        for flavor in &self.config.flavors {
            self.copy_template_file(
                "common/main_flavor.dart.tmpl",
                &self.config.output_dir.join(&format!("lib/main_{}.dart", flavor)),
                &[
                    ("{package_name}", &self.config.package_name),
                    ("{flavor}", flavor),
                    ("{flavor_title}", &(flavor[0..1].to_uppercase() + &flavor[1..]))
                ]
            )?;
            pb.inc(1);
        }
        
        // Create flavors.dart to define flavor enum
        self.copy_template_file(
            "common/flavors.dart",
            &self.config.output_dir.join("lib/flavors.dart"),
            &[("{flavors}", &self.config.flavors.join(", "))]
        )?;
        
        pb.finish_and_clear();
        println!("✅ Flavors setup completed");
        Ok(())
    }
    
    fn add_features(&self) -> Result<()> {
        println!("Adding selected features...");
        let pb = self.create_progress_bar(self.config.features.len() as u64);
        
        for feature in &self.config.features {
            match feature.as_str() {
                "auth" => {
                    create_auth_feature(&self.config.output_dir)?;
                },
                "notifications" => {
                    create_notification_feature(&self.config.output_dir)?;
                },
                "main_page" => {
                    create_main_page_feature(&self.config.output_dir)?;
                },
                _ => {
                    println!("Skipping unknown feature: {}", feature);
                }
            }
            pb.inc(1);
        }
        
        pb.finish_and_clear();
        println!("✅ All selected features added");
        Ok(())
    }
    
    fn update_pubspec(&self) -> Result<()> {
        println!("Updating pubspec.yaml...");
        
        // Read the current pubspec
        let pubspec_path = self.config.output_dir.join("pubspec.yaml");
        let mut pubspec_content = fs::read_to_string(&pubspec_path)
            .context("Failed to read pubspec.yaml")?;
        
        // Add dependencies
        let dependencies = r#"
  # State Management
  flutter_bloc: ^8.1.3
  equatable: ^2.0.5
  
  # Dependency Injection
  get_it: ^7.6.4
  injectable: ^2.3.2
  
  # Networking
  dio: ^5.3.3
  http: ^1.1.0
  connectivity_plus: ^5.0.1
  
  # Local Storage
  shared_preferences: ^2.2.2
  
  # Utilities
  dartz: ^0.10.1
  logger: ^2.0.2
  intl: ^0.18.1
  json_annotation: ^4.8.1
  
  # UI utilities
  cached_network_image: ^3.3.0
  flutter_svg: ^2.0.9
  shimmer: ^3.0.0"#;
        
        // Add development dependencies
        let dev_dependencies = r#"
  # Flutter flavorizr for handling flavors
  flutter_flavorizr: ^2.2.1
  
  # Code generation
  build_runner: ^2.4.6
  json_serializable: ^6.7.1
  injectable_generator: ^2.4.1"#;
        
        // Update pubspec content
        pubspec_content = pubspec_content.replace(
            "dependencies:\n  flutter:", 
            &format!("dependencies:\n  flutter:{}", dependencies)
        );
        
        pubspec_content = pubspec_content.replace(
            "dev_dependencies:\n  flutter_test:", 
            &format!("dev_dependencies:\n  flutter_test:{}", dev_dependencies)
        );
        
        // Write back to pubspec.yaml
        fs::write(&pubspec_path, pubspec_content)
            .context("Failed to update pubspec.yaml")?;
        
        println!("✅ pubspec.yaml updated");
        Ok(())
    }
    
    fn setup_assets(&self) -> Result<()> {
        println!("Setting up asset directories...");
        
        let asset_dirs = [
            "assets/fonts",
            "assets/i18n",
            "assets/images",
            "assets/colors",
        ];
        
        for dir in &asset_dirs {
            fs::create_dir_all(self.config.output_dir.join(dir))
                .context(format!("Failed to create asset directory: {}", dir))?;
        }
        
        // Copy placeholder assets
        self.copy_template_file(
            "assets/i18n/en.json",
            &self.config.output_dir.join("assets/i18n/en.json"),
            &[]
        )?;
        
        // Copy placeholder images
        self.copy_template_file(
            "assets/images/placeholder.png",
            &self.config.output_dir.join("assets/images/placeholder.png"),
            &[]
        )?;
        
        // Update pubspec to include assets
        let pubspec_path = self.config.output_dir.join("pubspec.yaml");
        let mut pubspec_content = fs::read_to_string(&pubspec_path)
            .context("Failed to read pubspec.yaml")?;
        
        let assets_section = r#"
  assets:
    - assets/i18n/
    - assets/images/
    
  fonts:
    - family: AppIcons
      fonts:
        - asset: assets/fonts/AppIcons.ttf"#;
        
        // Add assets section before the end of the file
        if !pubspec_content.contains("assets:") {
            pubspec_content.push_str(assets_section);
            fs::write(&pubspec_path, pubspec_content)
                .context("Failed to update pubspec.yaml with assets")?;
        }
        
        println!("✅ Asset directories created");
        Ok(())
    }
    
    fn add_flavorizr_config(&self) -> Result<()> {
        println!("Adding flavorizr configuration...");
        
        let pubspec_path = self.config.output_dir.join("pubspec.yaml");
        let mut pubspec_content = fs::read_to_string(&pubspec_path)
            .context("Failed to read pubspec.yaml")?;
        
        // Generate flavorizr config
        let mut flavor_configs = String::new();
        for flavor in &self.config.flavors {
            flavor_configs.push_str(&format!("\n    {}: {{}}", flavor));
        }
        
        let flavorizr_config = format!(r#"
flutter_flavorizr:
  flavors: {{{}}}"#, flavor_configs);
        
        // Add flavorizr config to pubspec
        pubspec_content.push_str(&flavorizr_config);
        fs::write(&pubspec_path, pubspec_content)
            .context("Failed to add flavorizr config to pubspec.yaml")?;
        
        println!("✅ Flavorizr configuration added");
        Ok(())
    }
    
    fn copy_template_dir(&self, template_subpath: &str, destination: &Path) -> Result<()> {
        crate::utils::copy_template_dir(template_subpath, destination)
    }
    
    fn copy_template_file(&self, template_path: &str, dest_path: &Path, replacements: &[(&str, &str)]) -> Result<()> {
        copy_template_file(template_path, dest_path, replacements)
    }
    
    fn create_progress_bar(&self, size: u64) -> ProgressBar {
        let pb = ProgressBar::new(size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"));
        pb
    }
}
