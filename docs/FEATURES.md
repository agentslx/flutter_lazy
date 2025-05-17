# Flutter Project Generator - Feature Structure

## Overview

This document describes the standard feature structure created by the Flutter Project Generator. Features in our Flutter architecture are organized in a modular, self-contained way, enabling clear separation of concerns and maintainable code.

## Feature Directory Structure

When a feature is generated, it follows this directory structure:

```
feature_name/
├── cubits/                    # State management
│   └── feature_cubit/         # Cubit with part-based approach
│       ├── feature_cubit.dart # Contains both cubit and part directive
│       └── feature_state.dart # Part file for state
├── data/                      # Data layer
│   ├── datasources/           # Data sources
│   │   ├── feature_local_datasource.dart
│   │   └── feature_remote_datasource.dart
│   ├── models/                # Data models
│   │   ├── enums.dart
│   │   ├── failure.dart
│   │   ├── feature_model.dart
│   │   └── feature_response_model.dart
│   └── repository/            # Repository pattern implementation
│       └── feature_repository.dart
├── ui/                        # Presentation layer
│   ├── pages/                 # Screen/pages
│   │   ├── feature_page.dart
│   │   └── feature_detail_page.dart
│   └── widgets/               # Reusable UI components
│       └── feature_item_widget.dart
├── di.dart                    # Dependency injection registration
└── router.dart                # Feature routing configuration
```

## Component Details

### State Management (Cubits)

The cubit pattern follows a modern implementation with these characteristics:

1. **Single-File Cubit with Part Files**:
   - `feature_cubit.dart`: Contains the cubit class and part declaration
   - `feature_state.dart`: Contains the state class as a part file

2. **State Structure**:
   - Immutable state class using Equatable
   - FormzSubmissionStatus for tracking loading states
   - Error and success message handling
   - Clean copyWith implementation for state updates

3. **Cubit Responsibilities**:
   - Initialization and data loading
   - CRUD operations
   - Business logic processing
   - Error handling

### Data Layer

#### Repository Pattern
- `feature_repository.dart`:
  - Abstract class defining the API contract
  - Implementation class with error handling
  - Either<Failure, Success> return type for error handling

#### Data Sources
- `feature_remote_datasource.dart`:
  - Abstract class for API contract
  - Implementation for API communication
  - Error handling for network requests

- `feature_local_datasource.dart`:
  - Abstract class for local storage contract
  - Implementation for cache management
  - Methods for CRUD operations on local data

#### Models
- `feature_model.dart`: Main data model with serialization
- `feature_response_model.dart`: API response structure
- `failure.dart`: Feature-specific failure types
- `enums.dart`: Feature-specific enumerations

### UI Layer

#### Pages
- `feature_page.dart`:
  - Main feature screen
  - BlocProvider + BlocConsumer pattern
  - Error and loading state handling

- `feature_detail_page.dart`:
  - Detail view for an individual item
  - Edit/delete functionality

#### Widgets
- `feature_item_widget.dart`:
  - List item representation
  - Reusable component with callback methods

### Configuration

- `di.dart`:
  - Dependency injection registration
  - Factory methods for feature dependencies

- `router.dart`:
  - Go Router implementation
  - Route definitions with parameters
  - Helper methods for navigation

## Design Principles

1. **Separation of Concerns**:
   - Each component has a single responsibility
   - The data layer doesn't depend on the presentation layer
   - Business logic is contained in cubits, not UI components

2. **Dependency Injection**:
   - All dependencies are registered and injected
   - Loose coupling between components
   - Testable code with mockable dependencies

3. **Error Handling**:
   - Consistent error handling with Either<Failure, Success>
   - Centralized error processing
   - User-friendly error display

4. **Immutability**:
   - Immutable state classes
   - Pure functions for state updates
   - Event-driven architecture

## Customization Options

The feature generator supports customizing which components are included:

- `--minimal`: Creates a minimal feature without state management and repositories
- `--no-state`: Skips state management files (cubit/bloc)
- `--no-repository`: Skips repository files
- `--no-models`: Skips model files
- `--no-pages`: Skips UI pages
- `--no-routing`: Skips routing configuration
- `--no-di`: Skips dependency injection setup
