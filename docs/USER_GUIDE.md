# Flutter Project Generator User Guide

## Installation

```bash
cd flutter_lazy
./install.sh
```

This will compile the tool and install it in your local environment.

## Commands

### Create a New Flutter Project

```bash
flutter_lazy new -n project_name
```

#### Options:
- `-n, --name`: Project name
- `-o, --output`: Output directory
- `-p, --package_name`: Package name (e.g., com.example.app)

If options are not provided, the tool will prompt for them interactively.

### Create a New Feature

```bash
flutter_lazy feature -n feature_name
```

#### Options:
- `-n, --name`: Feature name
- `-p, --project`: Project directory
- `-m, --minimal`: Create a minimal feature (fewer files)
- `--no-state`: Skip state management files
- `--no-repository`: Skip repository files
- `--no-models`: Skip model files
- `--no-pages`: Skip UI pages
- `--no-routing`: Skip routing configuration
- `--no-di`: Skip dependency injection setup

## Interactive Mode

If you run the commands without options, the tool will prompt you for the required information:

```bash
flutter_lazy feature
```

This will start an interactive prompt:

1. Select feature type (predefined or custom)
2. Enter the project directory
3. Choose between minimal or full feature
4. Select components to include
5. The tool will generate the feature with the selected options

## Feature Types

### Generic Features

Generic features are created with standard components:

```bash
flutter_lazy feature -n settings
```

### Specialized Features

Some features have specialized implementations with additional files:

```bash
flutter_lazy feature -n authentication
flutter_lazy feature -n notifications
flutter_lazy feature -n home_navigation
```

## Examples

### Create a Minimal Feature

```bash
flutter_lazy feature -n analytics -m
```

### Create a Feature with Selected Components

```bash
flutter_lazy feature -n user_profile --no-repository --no-models
```

### Create a Complete Feature

```bash
flutter_lazy feature -n settings
```

## Troubleshooting

### Template Not Found Errors

If you encounter "Template file not found" errors:

1. Check that the template directory is correct
2. Verify the template file exists at the expected path
3. Ensure the tool has read permissions for the templates
4. If using a custom template path, verify it's correctly specified

### Component Generation Issues

If a component is missing or incorrect:

1. Verify the feature was generated with the correct options
2. Check the component files in the template directory
3. Make sure the file naming convention matches the expected pattern

### Dependency Issues

If generated code has missing dependencies:

1. Make sure the project's pubspec.yaml has the required dependencies
2. Run `flutter pub get` after generating the feature
3. Verify that imports use the correct paths based on the project structure
