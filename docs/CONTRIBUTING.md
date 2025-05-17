# Flutter Project Generator - Contributing Guide

## Getting Started

1. Fork the repository
2. Clone your forked repository
3. Install Rust and Cargo if you haven't already
4. Install project dependencies with `cargo build`

## Project Structure

```
flutter_lazy/
├── src/                # Rust source code
├── templates/          # Template files
└── docs/               # Documentation
```

## Development Workflow

### Running the Project Locally

```bash
cargo run -- feature -n test_feature
```

### Building the Project

```bash
cargo build --release
```

### Testing New Features

1. Create a branch for your feature
2. Implement your changes
3. Test by generating sample features
4. Create a pull request

## Code Style Guidelines

- Follow Rust style conventions
- Use meaningful variable and function names
- Add comments to explain complex logic
- Structure code logically

## Modifying Templates

When modifying templates:

1. Keep the same placeholder variables
2. Follow the existing coding patterns
3. Test the templates with different feature names
4. Update documentation if needed

### Adding New Templates

1. Create the template file in the appropriate directory
2. Add the template to the generator logic in `features.rs`
3. Test the new template with a sample feature

## Pull Request Process

1. Create a branch for your changes
2. Make your changes with appropriate commits
3. Test your changes locally
4. Update documentation if required
5. Submit a pull request with a clear description
6. Address any review comments

## Versioning

We use semantic versioning:

- MAJOR: Incompatible API changes
- MINOR: New functionality in a backward-compatible manner
- PATCH: Backward-compatible bug fixes

## Common Development Tasks

### Adding a New Command

1. Add the command to the `Commands` enum in `main.rs`
2. Implement the command logic
3. Update the command handler in the `match` statement
4. Add documentation for the new command

### Adding Template Components

1. Create new template files
2. Update the generator to use these templates
3. Test with sample features
4. Update documentation

### Handling New Feature Types

1. Add a new feature type to the predefined features list
2. Create specialized template files if needed
3. Add a helper function in `features.rs`
4. Update the feature creation logic in `main.rs`

## Testing

Manually test the generator with various configurations:

- Different feature names (varying cases, special characters)
- Different component selections
- Different project structures
- Both minimal and full feature modes
