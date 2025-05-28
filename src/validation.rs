use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use serde_yaml;
use console::style;

// Structure to represent validation rules for project structure
#[derive(Debug, Deserialize, Serialize)]
pub struct ValidationRules {
    pub required_directories: Vec<String>,
    pub required_files: Vec<String>,
    pub file_content_checks: Vec<FileContentCheck>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileContentCheck {
    pub file_path: String,
    pub expected_content: Vec<String>,
}

pub struct ValidationResult {
    pub project_path: PathBuf,
    pub passed: bool,
    pub missing_directories: Vec<PathBuf>,
    pub missing_files: Vec<PathBuf>,
    pub failed_content_checks: Vec<(PathBuf, String)>,
}

impl ValidationResult {
    pub fn new(project_path: PathBuf) -> Self {
        ValidationResult {
            project_path,
            passed: true,
            missing_directories: Vec::new(),
            missing_files: Vec::new(),
            failed_content_checks: Vec::new(),
        }
    }

    pub fn print_summary(&self) {
        println!("\n{}", style("Validation Summary").bold().underlined());
        
        if self.passed {
            println!("{} All validations passed!", style("✅").green());
            return;
        }

        if !self.missing_directories.is_empty() {
            println!("{} Missing directories:", style("❌").red());
            for dir in &self.missing_directories {
                println!("  - {}", dir.display());
            }
        }

        if !self.missing_files.is_empty() {
            println!("{} Missing files:", style("❌").red());
            for file in &self.missing_files {
                println!("  - {}", file.display());
            }
        }

        if !self.failed_content_checks.is_empty() {
            println!("{} Failed content checks:", style("❌").red());
            for (file, reason) in &self.failed_content_checks {
                println!("  - {}: {}", file.display(), reason);
            }
        }
    }
}

pub struct ValidationSystem {
    rules: ValidationRules,
}

impl ValidationSystem {
    pub fn new(rules: ValidationRules) -> Self {
        ValidationSystem { rules }
    }

    /// Load validation rules from a file
    pub fn from_file(file_path: &Path) -> Result<Self> {
        let content = fs::read_to_string(file_path)
            .context(format!("Failed to read validation rules file: {}", file_path.display()))?;
            
        let rules: ValidationRules = serde_yaml::from_str(&content)
            .context("Failed to parse validation rules YAML")?;
            
        Ok(ValidationSystem { rules })
    }

    /// Create default validation rules for new project creation
    pub fn default_new_project_rules() -> ValidationRules {
        // Try to load from template file first
        let template_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("templates/validation/project_validation.yml");
            
        if template_path.exists() {
            if let Ok(content) = fs::read_to_string(&template_path) {
                if let Ok(rules) = serde_yaml::from_str::<ValidationRules>(&content) {
                    return rules;
                }
            }
        }
        
        // Fallback to hardcoded rules if template file doesn't exist or can't be parsed
        ValidationRules {
            required_directories: vec![
                "lib/config".to_string(),
                "lib/core/entities".to_string(),
                "lib/core/failures".to_string(),
                "lib/core/utils".to_string(),
                "lib/features".to_string(),
                "lib/modules/bloc".to_string(),
                "lib/modules/rest_module".to_string(),
                "lib/widgets/buttons".to_string(),
                "assets/images".to_string(),
                "assets/i18n".to_string(),
            ],
            required_files: vec![
                "lib/main.dart".to_string(),
                "lib/flavors.dart".to_string(),
                "lib/core/failures/failure.dart".to_string(),
                "lib/core/utils/logger.dart".to_string(),
                "pubspec.yaml".to_string(),
            ],
            file_content_checks: vec![
                FileContentCheck {
                    file_path: "pubspec.yaml".to_string(),
                    expected_content: vec![
                        "flutter_bloc:".to_string(),
                        "get_it:".to_string(),
                        "dio:".to_string(),
                        "assets:".to_string(),
                    ],
                },
                FileContentCheck {
                    file_path: "lib/flavors.dart".to_string(),
                    expected_content: vec![
                        "enum Flavor".to_string(),
                    ],
                },
            ],
        }
    }

    /// Create default validation rules for feature creation
    pub fn default_feature_rules(feature_name: &str) -> ValidationRules {
        // Try to load from template file first
        let template_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("templates/validation/feature_validation.yml");
            
        if template_path.exists() {
            if let Ok(content) = fs::read_to_string(&template_path) {
                // Replace {feature_name} placeholder with actual feature name
                let content = content.replace("{feature_name}", feature_name);
                
                if let Ok(rules) = serde_yaml::from_str::<ValidationRules>(&content) {
                    return rules;
                }
            }
        }
        
        // Fallback to hardcoded rules
        ValidationRules {
            required_directories: vec![
                format!("lib/features/{}/presentation", feature_name),
                format!("lib/features/{}/data", feature_name),
                format!("lib/features/{}/domain", feature_name),
            ],
            required_files: vec![
                format!("lib/features/{}/presentation/pages/{}_page.dart", feature_name, feature_name),
                format!("lib/features/{}/domain/repositories/{}_repository.dart", feature_name, feature_name),
            ],
            file_content_checks: vec![],
        }
    }

    /// Validate a project structure against the rules
    pub fn validate_project(&self, project_path: &Path) -> ValidationResult {
        let mut result = ValidationResult::new(project_path.to_path_buf());
        
        // Check required directories
        for dir_path in &self.rules.required_directories {
            let full_path = project_path.join(dir_path);
            if !full_path.exists() || !full_path.is_dir() {
                result.missing_directories.push(full_path);
                result.passed = false;
            }
        }
        
        // Check required files
        for file_path in &self.rules.required_files {
            let full_path = project_path.join(file_path);
            if !full_path.exists() || !full_path.is_file() {
                result.missing_files.push(full_path);
                result.passed = false;
            }
        }
        
        // Check file contents
        for check in &self.rules.file_content_checks {
            let full_path = project_path.join(&check.file_path);
            if full_path.exists() && full_path.is_file() {
                if let Ok(content) = fs::read_to_string(&full_path) {
                    for expected in &check.expected_content {
                        if !content.contains(expected) {
                            result.failed_content_checks.push((
                                full_path.clone(), 
                                format!("Missing expected content: '{}'", expected)
                            ));
                            result.passed = false;
                            break;
                        }
                    }
                } else {
                    result.failed_content_checks.push((
                        full_path, 
                        "Could not read file content".to_string()
                    ));
                    result.passed = false;
                }
            }
        }
        
        result
    }

    /// Validate a specific feature structure
    pub fn validate_feature(&self, project_path: &Path, _feature_name: &str) -> ValidationResult {
        self.validate_project(project_path)
    }

    /// Run validation and print results
    pub fn run_validation(&self, project_path: &Path) -> bool {
        let result = self.validate_project(project_path);
        result.print_summary();
        result.passed
    }
}

// Helper function to create custom validation rules
pub fn create_custom_validation(
    required_dirs: Vec<String>,
    required_files: Vec<String>,
    content_checks: Vec<FileContentCheck>
) -> ValidationRules {
    ValidationRules {
        required_directories: required_dirs,
        required_files: required_files,
        file_content_checks: content_checks,
    }
}

// Function to save validation rules to a file for future reference
pub fn save_validation_rules(rules: &ValidationRules, file_path: &Path) -> Result<()> {
    let yaml = serde_yaml::to_string(rules)
        .context("Failed to serialize validation rules to YAML")?;
        
    fs::write(file_path, yaml)
        .context(format!("Failed to write validation rules to {}", file_path.display()))?;
        
    Ok(())
}

// Custom validation for API feature generation
pub fn create_api_feature_validation(feature_name: &str) -> ValidationRules {
    // Try to load from template file first
    let template_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("templates/validation/api_feature_validation.yml");
        
    if template_path.exists() {
        if let Ok(content) = fs::read_to_string(&template_path) {
            // Replace {feature_name} placeholder with actual feature name
            let content = content.replace("{feature_name}", feature_name);
            
            if let Ok(rules) = serde_yaml::from_str::<ValidationRules>(&content) {
                return rules;
            }
        }
    }
    
    // Fallback to hardcoded rules
    ValidationRules {
        required_directories: vec![
            format!("lib/features/{}/data/models", feature_name),
            format!("lib/features/{}/data/datasources", feature_name),
            format!("lib/features/{}/data/repository", feature_name),
            format!("lib/core/entities/{}", feature_name),
        ],
        required_files: vec![
            format!("lib/features/{}/data/datasources/{}_remote_datasource.dart", feature_name, feature_name),
            format!("lib/features/{}/data/datasources/{}_local_datasource.dart", feature_name, feature_name),
            format!("lib/features/{}/data/repository/{}_repository.dart", feature_name, feature_name),
        ],
        file_content_checks: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    
    #[test]
    fn test_validation_rules_creation() {
        let rules = ValidationSystem::default_new_project_rules();
        assert!(!rules.required_directories.is_empty());
        assert!(!rules.required_files.is_empty());
        assert!(!rules.file_content_checks.is_empty());
    }
    
    #[test]
    fn test_feature_validation_rules() {
        let feature_name = "test_feature";
        let rules = ValidationSystem::default_feature_rules(feature_name);
        
        // Check if feature name is properly inserted in the paths
        for dir in &rules.required_directories {
            assert!(dir.contains(feature_name));
        }
        
        for file in &rules.required_files {
            assert!(file.contains(feature_name));
        }
    }
    
    #[test]
    fn test_api_feature_validation_rules() {
        let feature_name = "test_api";
        let rules = create_api_feature_validation(feature_name);
        
        // Check if feature name is properly inserted in the paths
        for dir in &rules.required_directories {
            assert!(dir.contains(feature_name));
        }
        
        for file in &rules.required_files {
            assert!(file.contains(feature_name));
        }
    }
    
    #[test]
    fn test_validation_result() {
        let temp_dir = PathBuf::from("/tmp/flutter_test");
        let result = ValidationResult::new(temp_dir.clone());
        
        // Initial state should be passing
        assert!(result.passed);
        assert!(result.missing_directories.is_empty());
        assert!(result.missing_files.is_empty());
        assert!(result.failed_content_checks.is_empty());
    }
}
