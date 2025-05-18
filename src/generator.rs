use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Context, Result};
use dialoguer::MultiSelect;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use convert_case::{Case, Casing};
use fs_extra::dir::{self, CopyOptions};

use crate::utils::copy_template_file;
use crate::features::{create_feature, create_auth_feature, create_notification_feature, create_main_page_feature};

pub struct ProjectConfig {
    pub name: String,
    pub package_name: String,
    pub flavors: Vec<String>,
    pub features: Vec<String>,
    pub output_dir: PathBuf,
}

pub struct FlutterProjectGenerator {
    config: ProjectConfig,
}

impl FlutterProjectGenerator {
    pub fn new(name: &str, output: &Option<PathBuf>, package_name: &Option<String>) -> Result<Self> {
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
        
        Ok(Self {
            config: ProjectConfig {
                name: name.to_string(),
                package_name,
                flavors,
                features: selected_features,
                output_dir: output_dir.join(project_dir_name),
            },
        })
    }
    
    pub fn generate(&self) -> Result<()> {
        self.create_base_project()?;
        self.setup_project_structure()?;
        self.create_flavors()?;
        self.add_features()?;
        self.update_pubspec()?;
        
        Ok(())
    }
    
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
            // Note: We're excluding 'generated' directory which will be created by build_runner
        ];
        
        let pb = self.create_progress_bar(directories.len() as u64);
        
        for dir in directories.iter() {
            std::fs::create_dir_all(lib_dir.join(dir))
                .context(format!("Failed to create directory: {}", dir))?;
            pb.inc(1);
        }
        
        // Create config files
        self.copy_template_file("common/app_structure/config/app_styles.dart.tmpl", &lib_dir.join("config/app_styles.dart"), &[])?;
        self.copy_template_file("common/app_structure/config/app_theme.dart.tmpl", &lib_dir.join("config/app_theme.dart"), &[])?;
        self.copy_template_file("common/app_structure/config/app_constants.dart.tmpl", &lib_dir.join("config/app_constants.dart"), &[])?;
        self.copy_template_file("common/app_structure/config/app_routes.dart.tmpl", &lib_dir.join("config/app_routes.dart"), &[])?;
        
        // Create core utility files
        self.copy_template_file("common/app_structure/core/extensions/string_extensions.dart.tmpl", &lib_dir.join("core/extensions/string_extensions.dart"), &[])?;
        self.copy_template_file("common/app_structure/core/extensions/context_extensions.dart.tmpl", &lib_dir.join("core/extensions/context_extensions.dart"), &[])?;
        self.copy_template_file("common/app_structure/core/utils/logger.dart.tmpl", &lib_dir.join("core/utils/logger.dart"), &[])?;
        self.copy_template_file("common/app_structure/core/utils/validators.dart.tmpl", &lib_dir.join("core/utils/validators.dart"), &[])?;
        self.copy_template_file("common/app_structure/core/failures/failure.dart.tmpl", &lib_dir.join("core/failures/failure.dart"), &[])?;
        self.copy_template_file("common/app_structure/core/failures/exceptions.dart.tmpl", &lib_dir.join("core/failures/exceptions.dart"), &[])?;
        
        // Create form validators
        self.copy_template_file("common/app_structure/core/form/email_input.dart.tmpl", &lib_dir.join("core/form/email_input.dart"), &[])?;
        self.copy_template_file("common/app_structure/core/form/password_input.dart.tmpl", &lib_dir.join("core/form/password_input.dart"), &[])?;
        self.copy_template_file("common/app_structure/core/form/name_input.dart.tmpl", &lib_dir.join("core/form/name_input.dart"), &[])?;
        self.copy_template_file("common/app_structure/core/form/confirmed_password_input.dart.tmpl", &lib_dir.join("core/form/confirmed_password_input.dart"), &[])?;
        
        // Create common widgets
        self.copy_template_file("common/app_structure/widgets/buttons/app_button.dart.tmpl", &lib_dir.join("widgets/buttons/app_button.dart"), &[])?;
        self.copy_template_file("common/app_structure/widgets/inputs/app_text_field.dart.tmpl", &lib_dir.join("widgets/inputs/app_text_field.dart"), &[])?;
        self.copy_template_file("common/app_structure/widgets/dialogs/app_dialog.dart.tmpl", &lib_dir.join("widgets/dialogs/app_dialog.dart"), &[])?;
        self.copy_template_file("common/app_structure/widgets/cards/info_card.dart.tmpl", &lib_dir.join("widgets/cards/info_card.dart"), &[])?;
        
        // Create modules files
        self.copy_template_file("common/app_structure/modules/local_storage_module/local_storage.dart.tmpl", &lib_dir.join("modules/local_storage_module/local_storage.dart"), &[])?;
        self.copy_template_file("common/app_structure/modules/local_storage_module/local_storage_module.dart.tmpl", &lib_dir.join("modules/local_storage_module/local_storage_module.dart"), &[])?;
        self.copy_template_file("common/app_structure/modules/local_storage_module/shared_pref_impl.dart.tmpl", &lib_dir.join("modules/local_storage_module/shared_pref_impl.dart"), &[])?;
        
        self.copy_template_file("common/app_structure/modules/rest_module/api_client.dart.tmpl", &lib_dir.join("modules/rest_module/api_client.dart"), &[])?;
        self.copy_template_file("common/app_structure/modules/rest_module/restful_module.dart.tmpl", &lib_dir.join("modules/rest_module/restful_module.dart"), &[])?;
        self.copy_template_file("common/app_structure/modules/rest_module/restful_module_dio_impl.dart.tmpl", &lib_dir.join("modules/rest_module/restful_module_dio_impl.dart"), &[])?;
        self.copy_template_file("common/app_structure/modules/rest_module/cancel_token.dart.tmpl", &lib_dir.join("modules/rest_module/cancel_token.dart"), &[])?;
        self.copy_template_file("common/app_structure/modules/rest_module/options.dart.tmpl", &lib_dir.join("modules/rest_module/options.dart"), &[])?;
        self.copy_template_file("common/app_structure/modules/rest_module/response.dart.tmpl", &lib_dir.join("modules/rest_module/response.dart"), &[])?;
        
        self.copy_template_file("common/app_structure/modules/bloc/bloc_observer.dart.tmpl", &lib_dir.join("modules/bloc/bloc_observer.dart"), &[])?;
        
        self.copy_template_file("common/app_structure/modules/push_notification/notification_module.dart.tmpl", &lib_dir.join("modules/push_notification/notification_module.dart"), &[])?;
        
        // Create helper files
        self.copy_template_file("common/app_structure/helpers/date_time_helper.dart.tmpl", &lib_dir.join("helpers/date_time_helper.dart"), &[])?;
        self.copy_template_file("common/app_structure/helpers/ui_helper.dart.tmpl", &lib_dir.join("helpers/ui_helper.dart"), &[])?;
        
        pb.finish_and_clear();
        println!("✅ Directory structure created");
        Ok(())
    }
    
    fn create_flavors(&self) -> Result<()> {
        println!("Setting up flavors...");
        if self.config.flavors.is_empty() {
            println!("No flavors selected, skipping");
            return Ok(());
        }
        
        let pb = self.create_progress_bar(2);
        
        // Create main_*.dart files for each flavor
        for flavor in &self.config.flavors {
            self.copy_template_file(
                &format!("common/main_flavor.dart.tmpl"),
                &self.config.output_dir.join("lib").join(format!("main_{}.dart", flavor)),
                &[("FLAVOR", flavor)]
            )?;
        }
        pb.inc(1);
        
        // Run flutter_flavorizr to setup flavors
        // First, update pubspec.yaml to include flavorizr configuration
        self.add_flavorizr_config()?;
        
        // Run flutter pub get and flutter pub run flutter_flavorizr
        Command::new("flutter")
            .current_dir(&self.config.output_dir)
            .args(&["pub", "get"])
            .output()
            .context("Failed to run flutter pub get")?;
            
        Command::new("flutter")
            .current_dir(&self.config.output_dir)
            .args(&["pub", "run", "flutter_flavorizr"])
            .output()
            .context("Failed to run flutter_flavorizr")?;
        
        pb.inc(1);
        
        pb.finish_and_clear();
        println!("✅ Flavors setup completed");
        Ok(())
    }
    
    fn add_features(&self) -> Result<()> {
        println!("Adding features...");
        let pb = self.create_progress_bar(self.config.features.len() as u64);
        
        for feature in &self.config.features {
            match feature.as_str() {
                "auth" => {
                    create_auth_feature(&self.config.output_dir)?;
                }
                "notifications" => {
                    create_notification_feature(&self.config.output_dir)?;
                }
                "main_page" => {
                    create_main_page_feature(&self.config.output_dir)?;
                }
                _ => println!("Unknown feature: {}", feature),
            }
            pb.inc(1);
        }
        
        pb.finish_and_clear();
        println!("✅ Features added");
        Ok(())
    }
    
    fn update_pubspec(&self) -> Result<()> {
        println!("Updating pubspec.yaml...");
        
        // Add required dependencies based on the architecture document
        let pubspec_path = self.config.output_dir.join("pubspec.yaml");
        let pubspec_content = std::fs::read_to_string(&pubspec_path)?;
        
        let mut new_pubspec = pubspec_content;
        
        // Add dependencies section with all the libraries mentioned in ARCHITECTURE.md
        let dependencies = r#"
  # State Management
  flutter_bloc: ^8.1.3
  equatable: ^2.0.5
  
  # Network
  dio: ^5.3.3
  pretty_dio_logger: ^1.3.1
  dartz: ^0.10.1
  
  # Dependency Injection
  get_it: ^7.6.4
  
  # Routing
  go_router: ^12.0.0
  
  # Storage
  shared_preferences: ^2.2.2
  path_provider: ^2.1.1
  
  # UI
  flutter_screenutil: ^5.9.0
  flutter_svg: ^2.0.7
  google_fonts: ^6.1.0
  flutter_adaptive_scaffold: ^0.1.7
  
  # Others
  easy_localization: ^3.0.3
  formz: ^0.6.1
  json_annotation: ^4.8.1
  logger: ^2.0.2+1
  sentry_flutter: ^7.10.1
  
  # Firebase
  firebase_core: ^2.16.0
  firebase_messaging: ^14.6.8
  
dev_dependencies:
  flutter_test:
    sdk: flutter
  
  # Linting
  flutter_lints: ^3.0.0
  
  # Code Generation
  build_runner: ^2.4.6
  json_serializable: ^6.7.1
  flutter_gen: ^5.3.2
  
  # Flavor Management
  flutter_flavorizr: ^2.2.1
"#;
        
        // Replace the original dependencies section with our custom one
        let deps_pattern = "dependencies:\n  flutter:\n    sdk: flutter";
        if new_pubspec.contains(deps_pattern) {
            new_pubspec = new_pubspec.replace(deps_pattern, &format!("dependencies:\n  flutter:\n    sdk: flutter{}", dependencies));
            
            // Remove the default dev_dependencies section to avoid duplication
            let dev_deps_pattern = "dev_dependencies:\n  flutter_test:\n    sdk: flutter\n\n  # The \"flutter_lints\"";
            if new_pubspec.contains(dev_deps_pattern) {
                // Find where the section ends (at the next top-level section)
                if let Some(dev_deps_start) = new_pubspec.find(dev_deps_pattern) {
                    if let Some(next_section) = new_pubspec[dev_deps_start..].find("\n# ") {
                        let section_end = dev_deps_start + next_section;
                        let before_section = &new_pubspec[0..dev_deps_start];
                        let after_section = &new_pubspec[section_end..];
                        new_pubspec = format!("{}{}", before_section, after_section);
                    }
                }
            }
            
            std::fs::write(&pubspec_path, new_pubspec)?;
        }
        
        println!("✅ Pubspec.yaml updated");
        Ok(())
    }
    
    fn setup_assets(&self) -> Result<()> {
        println!("Setting up assets...");
        
        // Create assets directory structure if it doesn't exist
        let assets_dir = self.config.output_dir.join("assets");
        let asset_dirs = [
            "icons", "images", "fonts", "i18n", 
            "firebase/dev", "firebase/stage", "firebase/prod"
        ];
        
        for dir in asset_dirs.iter() {
            std::fs::create_dir_all(assets_dir.join(dir))
                .context(format!("Failed to create asset directory: {}", dir))?;
        }
        
        // Copy placeholder assets from templates
        self.copy_template_dir("assets", &assets_dir)?;
        
        println!("✅ Assets setup completed");
        Ok(())
    }
    
    fn add_flavorizr_config(&self) -> Result<()> {
        println!("Configuring flavorizr...");
        
        let pubspec_path = self.config.output_dir.join("pubspec.yaml");
        let pubspec_content = std::fs::read_to_string(&pubspec_path)?;
        
        // Create flavorizr configuration
        let mut flavorizr_config = String::from("\nflutter_flavorizr:\n");
        flavorizr_config.push_str("  app:\n");
        flavorizr_config.push_str("    android:\n      flavorDimensions: \"flavor\"\n");
        flavorizr_config.push_str("    ios: {}\n\n");
        
        flavorizr_config.push_str("  flavors:\n");
        
        for flavor in &self.config.flavors {
            flavorizr_config.push_str(&format!("    {}:\n", flavor));
            flavorizr_config.push_str(&format!("      app:\n"));
            flavorizr_config.push_str(&format!("        name: \"{} {}\"\n", 
                self.config.name.to_case(Case::Title), flavor.to_case(Case::Title)));
                
            // Convert project name to package-compatible format
            let package_name_base = self.config.package_name.clone();
            flavorizr_config.push_str(&format!("      android:\n"));
            flavorizr_config.push_str(&format!("        applicationId: \"{}.{}\"\n", package_name_base, flavor));
            
            flavorizr_config.push_str(&format!("      ios:\n"));
            flavorizr_config.push_str(&format!("        bundleId: \"{}.{}\"\n", package_name_base, flavor));
        }
        
        // Append flavorizr configuration to pubspec.yaml
        let new_pubspec = format!("{}{}", pubspec_content, flavorizr_config);
        std::fs::write(pubspec_path, new_pubspec)?;
        
        println!("✅ Flavorizr configuration added");
        Ok(())
    }
    
    fn copy_template_dir(&self, template_subpath: &str, destination: &Path) -> Result<()> {
        let template_path = std::env::current_exe()?
            .parent()
            .context("Failed to get executable directory")?
            .join("templates")
            .join(template_subpath);
            
        // For development, check multiple local project paths
        let paths_to_check = vec![
            // Path when run from project root
            PathBuf::from("templates").join(template_subpath),
            // Path when run from inside project directory
            PathBuf::from("flutter_lazy/templates").join(template_subpath),
            // Current directory path
            PathBuf::from("./templates").join(template_subpath),
            // Path relative to workspace root
            PathBuf::from("../templates").join(template_subpath),
        ];
        
        // Try to find the template path
        let mut source_path = None;
        
        // First check the executable path
        if template_path.exists() {
            source_path = Some(template_path.clone());
        } else {
            // Then check all local paths
            for path in &paths_to_check {
                if path.exists() {
                    source_path = Some(path.clone());
                    break;
                }
            }
        }
        
        // Return error if no template found
        let source_path = source_path.ok_or_else(|| {
            eprintln!("Searched in:");
            eprintln!("  - {:?}", template_path);
            for path in &paths_to_check {
                eprintln!("  - {:?}", path);
            }
            anyhow::anyhow!("Template not found: {}", template_subpath)
        })?;
        
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
