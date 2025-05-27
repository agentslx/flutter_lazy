# Clean Architecture Implementation

## Overview

This document outlines how the Flutter Lazy tool now generates code following clean architecture principles:

1. **Entity Classes**: Domain models placed in `core/entities/{feature_name}` folder
2. **Data Models**: Feature-specific wrappers in `features/{feature_name}/data/models` for JSON conversion
3. **Repositories**: Return entities (not data models) to maintain domain independence
4. **Cubits**: Work with entities (not data models) to keep business logic clean

## Directory Structure

```
lib/
  ├── core/
  │   ├── entities/
  │   │   └── {feature_name}/
  │   │       └── {entity_name}.dart
  │   └── ...
  └── features/
      └── {feature_name}/
          ├── data/
          │   ├── models/
          │   │   └── {model_name}_model.dart
          │   ├── datasources/
          │   │   ├── {feature_name}_remote_datasource.dart
          │   │   └── {feature_name}_local_datasource.dart
          │   └── repository/
          │       └── {feature_name}_repository.dart
          └── cubits/
              └── {feature_name}_cubit/
                  ├── {feature_name}_cubit.dart
                  └── {feature_name}_state.dart
```

## Key Components

### Entity Classes

* Located in `core/entities/{feature_name}/{entity_name}.dart`
* Pure domain models without serialization logic
* Immutable with `copyWith` functionality

```dart
class Pet {
  final int id;
  final String name;
  final String category;
  
  const Pet({
    required this.id,
    required this.name,
    this.category,
  });
  
  Pet copyWith({
    int? id,
    String? name,
    String? category,
  }) {
    return Pet(
      id: id ?? this.id,
      name: name ?? this.name,
      category: category ?? this.category,
    );
  }
}
```

### Model Classes

* Located in `features/{feature_name}/data/models/{model_name}_model.dart`
* Handle JSON serialization/deserialization
* Include conversion methods to/from entities:

```dart
@JsonSerializable()
class PetModel {
  final int id;
  final String name;
  final String category;
  
  PetModel({
    required this.id,
    required this.name,
    this.category,
  });
  
  factory PetModel.fromJson(Map<String, dynamic> json) => _$PetModelFromJson(json);
  Map<String, dynamic> toJson() => _$PetModelToJson(this);
  
  // Convert to Entity
  Pet toEntity() {
    return Pet(
      id: id,
      name: name,
      category: category,
    );
  }
  
  // Create from Entity
  factory PetModel.fromEntity(Pet entity) {
    return PetModel(
      id: entity.id,
      name: entity.name,
      category: entity.category,
    );
  }
}
```

### Repository Implementation

* Located in `features/{feature_name}/data/repository/{feature_name}_repository.dart`
* Accepts models from data sources
* Converts models to entities before returning
* Uses Either type for error handling:

```dart
@Injectable(as: Pet_Repository)
class PetRepositoryImpl implements PetRepository {
  final PetRemoteDatasource _remoteDatasource;
  final PetLocalDatasource _localDatasource;

  PetRepositoryImpl(this._remoteDatasource, this._localDatasource);

  @override
  Future<Either<Failure, Pet>> getPetById(int id) async {
    try {
      final modelResult = await _remoteDatasource.getPetById(id);
      final entity = modelResult.toEntity();
      
      await _localDatasource.cachePetData(modelResult);
      return Right(entity);
    } catch (e) {
      return Left(UnexpectedFailure(message: e.toString()));
    }
  }
}
```

### Cubits/Presentation Layer

* Located in `features/{feature_name}/cubits/{feature_name}_cubit/`
* Work exclusively with entities, not data models:

```dart
class PetCubit extends Cubit<PetState> {
  final PetRepository _repository;

  PetCubit(this._repository) : super(PetInitial());

  Future<void> getPet(int id) async {
    emit(PetLoading());
    final result = await _repository.getPetById(id);
    result.fold(
      (failure) => emit(PetError(message: failure.message)),
      (pet) => emit(PetLoaded(pet: pet)),
    );
  }
}
```

## Clean Architecture Benefits

1. **Separation of Concerns**: Each layer has well-defined responsibilities
2. **Domain-Driven Design**: Entities represent pure business logic
3. **Testability**: Each layer can be tested independently
4. **Independence from External Changes**: UI and data sources can change without affecting business logic
5. **Maintainability**: Easier to understand and modify specific parts of the application
