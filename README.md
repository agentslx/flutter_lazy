# Flutter Lazy

A powerful CLI tool for bootstrapping Flutter projects with a clean, feature-first architecture based on best practices.

## Features

- **Interactive Setup**: Run the tool interactively with guided prompts
- **Feature-First Architecture**: Organize code by features rather than by layers
- **Multi-flavor Support**: Includes configuration for dev, stage, and production environments
- **Common Features**: Ready-to-use implementations for authentication, notifications, and navigation
- **Clean Architecture**: Properly separates concerns into presentation, domain, and data layers
- **State Management**: Integrates BLoC/Cubit for effective state management
- **Dependency Injection**: Sets up GetIt for service location and dependency injection
- **Navigation**: Implements go_router for declarative routing
- **Assets & Configurations**: Creates placeholder assets and Firebase configurations
- **Validation System**: Automatically validates project and feature structure against best practices

## Installation

### Quick Install

```bash
# Using the install script
curl -sSL https://raw.githubusercontent.com/username/flutter_lazy/main/install.sh | bash
```

### From Source

```bash
# Clone the repository 
git clone https://github.com/username/flutter_lazy.git
cd flutter_lazy

# Build the project
cargo build --release

# Install globally
cargo install --path .
```

## Usage

### Interactive Mode (Recommended)

Simply run without arguments for a guided, step-by-step setup:

```bash
# Create a new project interactively
flutter_lazy new

# Create a new feature interactively
flutter_lazy feature
```

### Create a New Project

```bash
# With all arguments specified
flutter_lazy new --name MyAwesomeApp --output ./projects --package-name com.example.myapp

# With minimal arguments (interactive prompts will ask for missing information)
flutter_lazy new --name MyAwesomeApp
```

The generator will:
1. Create a new Flutter project with the specified name
2. Set up the recommended directory structure based on ARCHITECTURE.md
3. Configure multi-flavor support (dev, stage, prod)
4. Add essential dependencies to pubspec.yaml
5. Generate boilerplate code for the core architecture
6. Add selected features (authentication, notifications, etc.)
7. Set up placeholder assets and configurations

### Create a New Feature

```bash
# Full feature with all components
flutter_lazy feature --name billing --project ./my_project

# Minimal feature (UI only, no state management or data layer)
flutter_lazy feature --name settings --minimal

# Custom configuration
flutter_lazy feature --name payments --no-repository --no-models
```

Each feature can include:
- **State Management**: BLoC/Cubit classes for managing UI state
- **Repository**: Data access layer with repository implementation
- **Models**: Data models and entity classes
- **UI Pages**: Screen layouts and widgets
- **Routing**: Navigation configuration for the feature
- **Dependency Injection**: Service locator setup

### Generate Features from API

```bash
# Generate features from a Swagger/OpenAPI URL
flutter_lazy from-api --url https://petstore.swagger.io/v2/swagger.json

# Generate features from a local Swagger/OpenAPI file with specific domains
flutter_lazy from-api --file ./swagger.json --domains users,products
```

### Validate Project Structure

```bash
# Validate entire project structure
flutter_lazy validate --project ./my_project

# Validate a specific feature
flutter_lazy validate --project ./my_project --feature authentication

# Validate a specific API feature
flutter_lazy validate --project ./my_project --api-feature users
```

You can also disable validation when creating projects or features:

```bash
# Skip validation when creating a new project
flutter_lazy new --name MyProject --no-validate

# Skip validation when creating a feature
flutter_lazy feature --name settings --no-validate
```

See [VALIDATION.md](./docs/VALIDATION.md) for more details about the validation system.

### Command Line Options

#### New Project

```
--name, -n          Project name (required if not using interactive mode)
--output, -o        Output directory (defaults to current directory)
--package-name, -p  Package name (e.g., com.example.app)
```

#### Feature

```
--name, -n          Feature name (required if not using interactive mode)
--project, -p       Project directory (defaults to current directory)
--minimal, -m       Create minimal feature (no state management or repositories)
--no-state          Skip state management files
--no-repository     Skip repository files
--no-models         Skip model files
--no-pages          Skip UI pages
--no-routing        Skip routing configuration
--no-di             Skip dependency injection setup
```

## Project Structure

### Overall Project Structure

The generated project follows this structure:

```
lib/
  ├── app.dart                    # Main application widget
  ├── main.dart                   # Default entry point
  ├── main_dev.dart               # Development flavor entry
  ├── main_stage.dart             # Staging flavor entry
  ├── main_production.dart        # Production flavor entry  
  ├── di.dart                     # Main dependency injection setup
  ├── router.dart                 # Main application router
  ├── flavors.dart                # Flavor configuration
  ├── firebase_options.dart       # Firebase configuration
  ├── config/                     # App-wide configuration
  │   ├── app_config.dart         # Environment-specific config
  │   ├── theme.dart              # App theme definition
  │   └── constants.dart          # App-wide constants
  ├── entities/                   # Domain entity models
  ├── features/                   # Feature modules (feature-first architecture)
  ├── generated/                  # Generated code (assets, localization)
  │   ├── assets.gen.dart         # Generated asset references
  │   ├── colors.gen.dart         # Generated color constants
  │   └── l10n/                   # Localization
  ├── helpers/                    # Utility functions
  ├── models/                     # Shared data models
  ├── modules/                    # Core modules (shared functionality)
  │   ├── bloc/                   # Bloc utilities
  │   ├── local_storage/          # Local storage functionality
  │   ├── push_notification/      # Push notification handling
  │   └── rest_client/            # Network request handling
  └── widgets/                    # Shared UI components
      ├── buttons/                # Button components
      ├── dialogs/                # Dialog components
      ├── inputs/                 # Input field components
      └── layout/                 # Layout components
```

### Feature Structure

Each feature follows the clean architecture pattern and is organized as follows:

```
features/
  └── feature_name/              # e.g., authentication, settings, etc.
      ├── data/                  # Data layer
      │   ├── datasources/       # Remote and local data sources
      │   ├── models/            # Data transfer objects
      │   └── repositories/      # Repository implementations
      ├── domain/                # Domain layer
      │   ├── entities/          # Business models
      │   ├── repositories/      # Repository interfaces
      │   └── usecases/          # Business use cases
      ├── presentation/          # Presentation layer
      │   ├── bloc/              # State management
      │   │   ├── feature_bloc.dart
      │   │   ├── feature_event.dart
      │   │   └── feature_state.dart
      │   ├── pages/             # Screen implementations
      │   └── widgets/           # Feature-specific widgets
      ├── di.dart                # Feature-specific dependency injection
      └── router.dart            # Feature routes
```

### Assets Structure

```
assets/
  ├── colors/                    # Color definitions
  ├── firebase/                  # Firebase configuration files
  │   ├── dev/                   # Dev environment configs
  │   ├── stage/                 # Staging environment configs
  │   └── prod/                  # Production environment configs
  ├── fonts/                     # Custom fonts
  ├── i18n/                      # Localization files
  ├── icons/                     # App icons
  └── images/                    # Images and illustrations
```

## Templates

The project generator uses templates based on best practices for each component. These templates include:

1. **Project Templates**: Base structure and common files
2. **Feature Templates**: Ready-made feature implementations (auth, notifications, etc.)
3. **Module Templates**: Core modules like networking, storage, etc.

## Customization

You can customize the generated project in several ways:

### 1. Modifying Templates

The templates are located in the `templates/` directory. You can modify them to match your team's coding style and preferences:

```
templates/
  ├── common/                  # Project-wide templates
  │   ├── app.dart.tmpl        # Main app template
  │   ├── router.dart.tmpl     # Router template
  │   └── ...
  ├── features/                # Feature templates
  │   ├── common/              # Shared feature templates
  │   ├── authentication/      # Auth feature templates
  │   └── ...
  └── assets/                  # Asset templates
      ├── firebase/            # Firebase config templates
      └── ...
```

### 2. Adding Custom Features

You can create your own feature templates in the `templates/features/` directory following the existing pattern.

### 3. Extending the Generator

The generator is built in a modular way, making it easy to extend:

1. Add new command in `src/main.rs`
2. Implement the feature in a dedicated module
3. Create appropriate templates in the `templates/` directory

## Advanced Usage

### Using Predefined Features

The generator includes several predefined features that implement common functionality:

#### Authentication

```bash
flutter_lazy feature --name authentication
```

Includes:
- Login/Registration flows
- User repository and models
- Authentication state management
- Secure token storage
- Profile page

#### Notifications

```bash
flutter_lazy feature --name notifications
```

Includes:
- Push notification setup
- Notification permissions handling
- Notification display and routing
- Background message handling
- Notification preferences

#### Home Navigation

```bash
flutter_lazy feature --name home_navigation
```

Includes:
- Bottom navigation bar
- Drawer navigation
- Tab navigation
- Deep linking support
- Route guards

### Batch Generation

You can combine commands to quickly set up a complete project:

```bash
# Create a new project with all the essentials
flutter_lazy new --name MyApp && cd MyApp && \
flutter_lazy feature --name authentication && \
flutter_lazy feature --name settings --minimal
```

## Requirements

- Rust 1.51.0 or higher
- Flutter 3.0.0 or higher
- Dart 2.17.0 or higher

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by the clean architecture principles of Robert C. Martin
- Based on Flutter community best practices
- Follows the feature-first approach recommended for large-scale applications

1. Modifying the templates in the `templates` directory
2. Adding new feature templates
3. Updating the dependencies in the generator code

## Requirements

- Flutter SDK
- Rust (for building from source)
- Cargo (Rust's package manager)

## License

MIT
