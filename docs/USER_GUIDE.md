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
- `--no-services`: Skip services
- `--no-utils`: Skip utils directory
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

### Create a UI-Only Feature with Services

```bash
flutter_lazy feature -n settings --no-state --no-repository --no-models --no-utils
```

### Generate Features from API Specification

```bash
flutter_lazy from-api -u https://petstore.swagger.io/v2/swagger.json
```

## Generate Features from Swagger/OpenAPI Specification

This feature allows you to automatically generate Flutter feature modules based on a Swagger/OpenAPI specification.

### Command

```bash
flutter_lazy from-api [options]
```

#### Options:
- `-u, --url`: URL to the Swagger/OpenAPI JSON specification
- `-f, --file`: Path to a local Swagger/OpenAPI JSON file
- `-p, --project`: Project directory
- `-d, --domains`: Only generate specific domains/tags (comma-separated)
- `--data-only`: Skip generating cubits/state management (data layer only, default: true)

### Interactive Mode

If you run the command without options, the tool will prompt you for the required information:

```bash
flutter_lazy from-api
```

### What Gets Generated

For each API domain (tag) in the Swagger specification, the tool will generate:

1. **Model classes** - Based on API response schemas
   - Properly typed Dart classes with JsonSerializable annotations
   - Support for nullable fields based on required properties

2. **Remote datasource** - For API communication
   - Methods for each endpoint in the domain
   - Proper error handling with Dio
   - Type-safe return values

3. **Local datasource** - For local caching
   - Basic implementation with SharedPreferences
   - Cache and retrieval methods

4. **Repository** - Business logic layer
   - Methods for each endpoint with proper error handling
   - Either type for success/failure results using dartz
   - Integration between remote and local datasources

5. **Base feature structure** - Standard folder organization

### Examples

#### Generate Features from a Remote Swagger API

```bash
flutter_lazy from-api -u https://petstore.swagger.io/v2/swagger.json
```

#### Generate Features from a Local Swagger File

```bash
flutter_lazy from-api -f ./api-docs.json
```

#### Generate Only Specific Domains

```bash
flutter_lazy from-api -u https://petstore.swagger.io/v2/swagger.json -d "pet,store,user"
```

#### Generate Complete Features with State Management

```bash
flutter_lazy from-api -u https://petstore.swagger.io/v2/swagger.json --data-only=false
```

### Create a New Project with API Integration

When creating a new project, you can now include Swagger/OpenAPI specifications directly during the project creation process:

```bash
flutter_lazy new -n project_name --api_url https://petstore.swagger.io/v2/swagger.json
```

Alternatively, you can use a local file:

```bash
flutter_lazy new -n project_name --api_file ./path/to/swagger.json
```

#### Options for API Integration:
- `--api_url`: Initialize with a Swagger/OpenAPI spec URL
- `--api_file`: Initialize with a local Swagger/OpenAPI spec file

If these options are not provided, the tool will interactively ask if you want to include an API specification during project setup.

The API integration offers several features:
- Interactive workflow with retry options if loading fails
- Option to filter specific domains/tags from the API
- Automatic generation of models, datasources, and repositories based on the API specification

This integration makes it simple to bootstrap your Flutter project with API-ready components from day one.
