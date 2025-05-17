# Flutter Project Generator Documentation

## Overview

The Flutter Project Generator is a command-line tool that streamlines the creation of Flutter projects and features following our standardized architecture patterns. This tool ensures consistent code organization and implementation across your Flutter application.

## Documentation Index

- [User Guide](USER_GUIDE.md): Instructions for using the tool
- [Architecture](ARCHITECTURE.md): Overview of the tool's architecture
- [Templates](TEMPLATES.md): Details on the template system
- [Features](FEATURES.md): Structure of generated features
- [Migration Guide](MIGRATION_GUIDE.md): How to migrate to newer template formats
- [Contributing](CONTRIBUTING.md): Guide for contributors

## Quick Start

### Installation

```bash
cd flutter_lazy
./install.sh
```

### Creating a New Feature

```bash
flutter_lazy feature -n feature_name
```

### Interactive Mode

```bash
flutter_lazy feature
```

Follow the interactive prompts to configure your feature.

## Key Features

- **Standardized Architecture**: Enforces consistent code organization
- **Customizable Generation**: Select which components to include
- **Template System**: Uses templates with placeholders for code generation
- **Specialized Features**: Support for common feature types
- **Interactive Mode**: User-friendly command-line interface

## Support

For bug reports and feature requests, please open an issue in the project repository.
