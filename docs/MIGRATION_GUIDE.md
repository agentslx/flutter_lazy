# Flutter Project Generator - Template Migration Guide

## Overview

This guide outlines the process for migrating from the old template format to the new folder-based structure with part files for cubits. Recent updates have introduced a new template structure that provides more maintainable and cleaner code organization.

## Template Changes

### State Management Templates

#### Old Structure (Legacy)
```
features/common/cubits/
├── feature_cubit.dart.tmpl
└── feature_state.dart.tmpl
```

#### New Structure (Current)
```
features/common/cubits/
├── feature_cubit/
│   ├── feature_cubit.dart.tmpl
│   └── feature_state.dart.tmpl
└── feature_cubit.dart.tmpl (Backward compatibility)
└── feature_state.dart.tmpl (Backward compatibility)
```

### Key Differences

1. **Part-Based Approach**:
   - New templates use the `part` directive for state files
   - State code is now located in a part file of the cubit
   - Cubit and state share the same file namespace

2. **FormzSubmissionStatus**:
   - Replacing custom state classes with FormzSubmissionStatus
   - Unified approach for loading, success, and error states

3. **Immutable State Management**:
   - Enhanced copyWith implementation
   - More comprehensive state properties
   - Better support for collections and complex state

## Migration Steps

### For Tool Maintainers

To update the code generator to use the new templates:

1. Modify the `features.rs` file to update the template paths:

```rust
// Before
copy_template_file(
    "features/common/cubits/feature_cubit.dart.tmpl",
    &feature_dir.join("cubits").join(format!("{}_cubit.dart", snake_name)),
    &replacements
)?;

// After
copy_template_file(
    "features/common/cubits/feature_cubit/feature_cubit.dart.tmpl",
    &feature_dir.join("cubits").join(format!("{}_cubit", snake_name)).join(format!("{}_cubit.dart", snake_name)),
    &replacements
)?;
```

2. Update the directory creation logic:

```rust
// Before
fs::create_dir_all(feature_dir.join("cubits"))
    .context("Failed to create cubits directory")?;

// After
fs::create_dir_all(feature_dir.join("cubits").join(format!("{}_cubit", snake_name)))
    .context("Failed to create cubit directory")?;
```

### Backward Compatibility

To maintain backward compatibility:

1. Keep duplicate templates in both locations
2. In the old location templates, use a part-based structure that references the state file in the same directory
3. Ensure the generator can handle both structures

## Testing Migration

After updating templates:

1. Generate a test feature with the new templates
2. Verify that the cubit directory structure is created correctly
3. Check that part files are properly linked
4. Confirm imports and exports are working as expected

## Common Migration Issues

1. **Template Path Errors**:
   - Error: "Template file not found"
   - Solution: Update path references in features.rs

2. **Import Path Errors**:
   - Error: "Target of URI doesn't exist"
   - Solution: Update import paths in templates to match the new structure

3. **Part File Registration**:
   - Error: "Part file is not registered to any library"
   - Solution: Ensure the part directive in the cubit file matches the state file path
