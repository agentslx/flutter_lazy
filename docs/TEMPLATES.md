# Flutter Project Generator - Template System

## Overview

The template system is the core of the Flutter Project Generator, enabling the creation of consistent, standardized code across features. This document explains how the template system works and how to create or modify templates.

## Template Location

Templates are stored in the `templates` directory and organized into categories:

```
templates/
├── common/            # Project-level templates
│   └── ...
└── features/          # Feature-specific templates
    ├── common/        # Generic feature templates
    │   ├── cubits/    # State management templates
    │   ├── data/      # Data layer templates
    │   └── ui/        # UI templates
    ├── auth/          # Auth feature templates
    └── ...            # Other specialized feature templates
```

## Template Format

Templates are regular Dart files with the `.tmpl` extension, containing placeholder tokens that are replaced during generation. For example:

```dart
// file: templates/features/common/cubits/feature_cubit/feature_cubit.dart.tmpl
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:formz/formz.dart';

import '../../../../di.dart';
import '../../data/repository/{{ FEATURE_NAME_SNAKE }}_repository.dart';

part '{{ FEATURE_NAME_SNAKE }}_state.dart';

class {{ FEATURE_NAME_PASCAL }}Cubit extends Cubit<{{ FEATURE_NAME_PASCAL }}State> {
  // Template contents...
}
```

## Placeholder Tokens

The template system uses these main tokens:

| Token | Description | Example Input | Example Output |
|-------|-------------|---------------|---------------|
| `{{ FEATURE_NAME_PASCAL }}` | PascalCase feature name | "user_profile" | "UserProfile" |
| `{{ FEATURE_NAME_SNAKE }}` | snake_case feature name | "userProfile" | "user_profile" |
| `{{ FEATURE_NAME_CAMEL }}` | camelCase feature name | "UserProfile" | "userProfile" |

## Template Types

### Cubit Templates

State management templates support both class-per-file and part-file approaches:

1. **Part-File Approach** (Preferred):
   - `cubits/feature_cubit/feature_cubit.dart.tmpl`: Main cubit file with part directive
   - `cubits/feature_cubit/feature_state.dart.tmpl`: State as part file

2. **Class-Per-File Approach** (Legacy):
   - `cubits/feature_cubit.dart.tmpl`: Cubit implementation
   - `cubits/feature_state.dart.tmpl`: State implementation

### Repository Templates

- `data/repository/feature_repository.dart.tmpl`: Repository interface and implementation

### Data Source Templates

- `data/datasources/feature_remote_datasource.dart.tmpl`: Remote data source
- `data/datasources/feature_local_datasource.dart.tmpl`: Local data source

### Model Templates

- `data/models/feature_model.dart.tmpl`: Main data model
- `data/models/feature_response_model.dart.tmpl`: API response models
- `data/models/failure.dart.tmpl`: Feature-specific failure classes
- `data/models/enums.dart.tmpl`: Feature-specific enumerations

### UI Templates

- `ui/pages/feature_page.dart.tmpl`: Main feature page
- `ui/pages/feature_detail_page.dart.tmpl`: Detail page
- `ui/widgets/feature_item_widget.dart.tmpl`: List item widget

### Configuration Templates

- `di.dart.tmpl`: Dependency injection setup
- `router.dart.tmpl`: Routing configuration

## File Naming Conventions

Templates follow these naming conventions:

1. Base template files use generic names like `feature_cubit.dart.tmpl`
2. When generating files, `feature` is replaced with the feature name in snake_case
3. Final generated files have names like `user_profile_cubit.dart`

## Template Updating Workflow

When updating templates:

1. Modify template files in the appropriate directories
2. Test by generating a new feature with the updated templates
3. Verify the generated code follows the desired architecture patterns
4. Update documentation if the template architecture changes

## Best Practices

1. Keep templates modular and focused on a single responsibility
2. Use consistent naming and structure across templates
3. Add comments in templates to explain complex patterns
4. Use proper import paths with placeholders
5. Make templates resilient to different feature names by properly using the placeholder tokens
