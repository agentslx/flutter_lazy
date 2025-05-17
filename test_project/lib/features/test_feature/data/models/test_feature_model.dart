import 'package:equatable/equatable.dart';
import 'package:json_annotation/json_annotation.dart';

part 'test_feature_model.g.dart';

@JsonSerializable(fieldRename: FieldRename.snake)
class TestFeatureModel extends Equatable {
  const TestFeatureModel({
    required this.id,
    required this.title,
    required this.description,
    required this.createdAt,
    this.updatedAt,
    this.isActive = true,
    this.metadata,
  });
  
  factory TestFeatureModel.fromJson(Map<String, dynamic> json) => 
      _$TestFeatureModelFromJson(json);
  
  /// Unique identifier for this item
  final String id;
  
  /// Title of the item
  final String title;
  
  /// Detailed description of the item
  final String description;
  
  /// Timestamp when the item was created
  final DateTime createdAt;
  
  /// Timestamp when the item was last updated
  final DateTime? updatedAt;
  
  /// Whether this item is active
  final bool isActive;
  
  /// Additional metadata associated with this item
  final Map<String, dynamic>? metadata;
      
  Map<String, dynamic> toJson() => _$TestFeatureModelToJson(this);
  
  /// Create a copy of this model with the given fields replaced with the new values
  TestFeatureModel copyWith({
    String? id,
    String? title,
    String? description,
    DateTime? createdAt,
    DateTime? updatedAt,
    bool? isActive,
    Map<String, dynamic>? metadata,
  }) {
    return TestFeatureModel(
      id: id ?? this.id,
      title: title ?? this.title,
      description: description ?? this.description,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
      isActive: isActive ?? this.isActive,
      metadata: metadata ?? this.metadata,
    );
  }
  
  @override
  List<Object?> get props => [
    id, 
    title, 
    description, 
    createdAt, 
    updatedAt, 
    isActive,
    metadata,
  ];
}
