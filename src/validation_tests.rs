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
