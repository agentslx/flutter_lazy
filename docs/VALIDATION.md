# Flutter Lazy Validation System

The validation system in Flutter Lazy helps ensure that generated projects and features comply with the expected structure and contain all necessary files. This is particularly useful for ensuring consistent project architecture and detecting potential issues early.

## Features

- **Project Structure Validation**: Verifies that the generated project has the expected directory structure, essential files, and required content.
- **Feature Structure Validation**: Confirms that generated features follow the correct architectural patterns.
- **API Feature Validation**: Ensures that features generated from Swagger/OpenAPI specifications have appropriate data models, repositories, and datasources.
- **Configurable Rules**: Validation rules are loaded from YAML templates that can be customized.

## Using Validation

Validation runs automatically after generating a project or feature. If you want to skip validation, use the `--no-validate` flag with any command:

```bash
# Skip validation when creating a new project
flutter_lazy new --name my_project --no-validate

# Skip validation when creating a feature
flutter_lazy feature --name my_feature --no-validate

# Skip validation when generating API features
flutter_lazy from-api --url https://example.com/swagger.json --no-validate
```

## Validation Rules

Validation rules are defined in YAML files located in the `templates/validation/` directory:

- `project_validation.yml`: Rules for validating complete projects
- `feature_validation.yml`: Rules for validating individual features
- `api_feature_validation.yml`: Rules for validating features generated from APIs

### Example Validation Rule File

```yaml
required_directories:
  - lib/core/entities
  - lib/core/failures
  - lib/features

required_files:
  - lib/main.dart
  - lib/flavors.dart
  - pubspec.yaml

file_content_checks:
  - file_path: pubspec.yaml
    expected_content:
      - flutter_bloc:
      - get_it:
  - file_path: lib/flavors.dart
    expected_content:
      - enum Flavor
```

## Custom Validation

You can create custom validation rules by creating or modifying the YAML files in the `templates/validation/` directory. Each validation rule file has three main sections:

1. `required_directories`: List of directories that must exist
2. `required_files`: List of files that must exist
3. `file_content_checks`: Content checks for specific files

For feature validation, use `{feature_name}` as a placeholder that will be replaced with the actual feature name.

## Validation Results

After validation runs, you'll see a summary of the results, including:

- ✅ Successful validation
- ❌ Missing directories or files
- ⚠️ Failed content checks

If validation fails, the tool will point out specific issues that need to be addressed.
