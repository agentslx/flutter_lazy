part of 'auth_cubit.dart';

class AuthState extends Equatable {
  const AuthState({
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

  /// List of Auth items
  final List<AuthModel> items;
  
  /// Currently selected item (for detail view)
  final AuthModel? selectedItem;
  
  /// Item being edited
  final AuthModel? editingItem;
  
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

  AuthState copyWith({
    List<AuthModel>? items,
    AuthModel? selectedItem,
    AuthModel? editingItem,
    FormzSubmissionStatus? status,
    String? errorMessage,
    String? successMessage,
    bool? isFiltering,
    String? searchQuery,
    Map<String, dynamic>? filterOptions,
  }) {
    return AuthState(
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
