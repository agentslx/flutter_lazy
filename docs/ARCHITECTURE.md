# Flutter Project Generator Architecture

## Overview

The Flutter Project Generator is a command-line tool written in Rust that helps create standardized Flutter projects and features following our architecture patterns. The tool provides consistent scaffolding for features, ensuring standardized code organization and implementation approaches.

## Project Structure

```
flutter_lazy/
├── src/                # Rust source code
│   ├── main.rs         # Entry point and CLI definitions
│   ├── generator.rs    # Core project generation logic
│   ├── features.rs     # Feature generation implementation
│   └── utils.rs        # Utility functions
├── templates/          # Template files for code generation
│   ├── common/         # Common project templates
│   └── features/       # Feature-specific templates
├── docs/               # Documentation
├── Cargo.toml          # Rust dependencies
└── install.sh          # Installation script
```

## Component Architecture

### Command Line Interface (main.rs)

The CLI is built using the `clap` Rust library, offering an intuitive command-line interface with subcommands:

1. `new`: Creates a new Flutter project with the predefined architecture
2. `feature`: Adds a new feature to an existing project

Each command supports various flags and options for customization.

### Project Generator (generator.rs)

Responsible for creating new Flutter projects with the following capabilities:

- Initializing a Flutter project using the `flutter create` command
- Structuring the project directories according to our architectural pattern
- Setting up multi-flavor configurations using `flutter_flavorizr`
- Adding common dependencies to the `pubspec.yaml`
- Copying template files for core functionality

### Feature Generator (features.rs)

Handles the generation of new features with these capabilities:

- Creating the feature directory structure
- Generating feature-specific files using templates
- Supporting both generic and specialized features
- Customizing the feature components based on user selection

### Templates

Templates use a simple placeholder substitution mechanism where tokens like `{{ FEATURE_NAME_PASCAL }}` or `{{ FEATURE_NAME_SNAKE }}` are replaced with the appropriate casing of the feature name.

## Workflow Architecture

1. **User Invokes Command**: The user runs the tool with appropriate parameters
2. **Parameter Processing**: The tool processes command-line arguments and collects missing information interactively
3. **Template Selection**: Templates are selected based on the type of feature and components requested
4. **Code Generation**: Templates are processed with appropriate replacements and copied to the target directory
5. **Finishing**: The tool completes by displaying success messages and next steps

## Technology Stack

- **Rust**: Core language for the CLI tool
- **clap**: Command-line argument parsing
- **dialoguer**: Interactive prompts
- **convert_case**: Case conversion for naming
- **anyhow**: Error handling
- **Template System**: Custom template processing using simple token replacement
