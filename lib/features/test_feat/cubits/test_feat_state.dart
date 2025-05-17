part of 'test_feat_cubit.dart';

class TestFeatState extends Equatable {
  const TestFeatState({
    this.items = const [],
    this.selectedItem,
    this.editingItem,
    this.status = FormzSubmissionStatus.initial,
    this.errorMessage,
    this.successMessage,
    this.isFiltering = false,
    this.searchQuery,
    this.filterOptions = const {},
  });

  /// List of TestFeat items
  final List<TestFeatModel> items;
  
  /// Currently selected item (for detail view)
  final TestFeatModel? selectedItem;
  
  /// Item being edited
  final TestFeatModel? editingItem;
  
  /// Current submission status
  final FormzSubmissionStatus status;
  
  /// Error message to display
  final String? errorMessage;
  
  /// Success message to display
  final String? successMessage;
  
  /// Whether filtering is active
  final bool isFiltering;
  
  /// Current search query
  final String? searchQuery;
  
  /// Filter options
  final Map<String, dynamic> filterOptions;

  /// Check if all required fields are valid 
  bool get isValid => true;
  
  /// Helper to check if there are any items
  bool get hasItems => items.isNotEmpty;

  @override
  List<Object?> get props => [
    items,
    selectedItem,
    editingItem,
    status,
    errorMessage,
    successMessage,
    isFiltering,
    searchQuery,
    filterOptions,
  ];

  TestFeatState copyWith({
    List<TestFeatModel>? items,
    TestFeatModel? selectedItem,
    TestFeatModel? editingItem,
    FormzSubmissionStatus? status,
    String? errorMessage,
    String? successMessage,
    bool? isFiltering,
    String? searchQuery,
    Map<String, dynamic>? filterOptions,
  }) {
    return TestFeatState(
      items: items ?? this.items,
      selectedItem: selectedItem ?? this.selectedItem,
      editingItem: editingItem ?? this.editingItem,
      status: status ?? this.status,
      errorMessage: errorMessage ?? this.errorMessage,
      successMessage: successMessage ?? this.successMessage,
      isFiltering: isFiltering ?? this.isFiltering,
      searchQuery: searchQuery ?? this.searchQuery,
      filterOptions: filterOptions ?? this.filterOptions,
    );
  }
}

// For backwards compatibility with older code
class TestFeatInitial extends TestFeatState {}
class TestFeatLoading extends TestFeatState {}
class TestFeatLoaded extends TestFeatState {
  final TestFeatModel data;
  const TestFeatLoaded(this.data) : super(items: [data]);
}
class TestFeatError extends TestFeatState {
  final String message;
  const TestFeatError(this.message) : super(errorMessage: message, status: FormzSubmissionStatus.failure);
}
