import 'dart:developer';

import 'package:easy_localization/easy_localization.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:formz/formz.dart';

import '../../../../di.dart';
import '../../data/models/auth_model.dart';
import '../../data/repository/auth_repository.dart';

part 'auth_state.dart';

class AuthCubit extends Cubit<AuthState> {
  AuthCubit() : super(const AuthState());

  final AuthRepository _repository = getIt<AuthRepository>();

  /// Initialize the cubit and load data
  Future<void> initialize() async {
    if (state.status.isInProgress) return;
    emit(state.copyWith(status: FormzSubmissionStatus.inProgress));

    try {
      final result = await _repository.getData();
      result.fold(
        (failure) {
          emit(
            state.copyWith(
              status: FormzSubmissionStatus.failure,
              errorMessage: failure.message,
            ),
          );
        },
        (data) {
          emit(state.copyWith(
            status: FormzSubmissionStatus.success,
            items: [data],
          ));
        },
      );
    } catch (e, stackTrace) {
      log('Error in initialize', error: e, stackTrace: stackTrace);
      emit(state.copyWith(
        status: FormzSubmissionStatus.failure,
        errorMessage: e.toString(),
      ));
    }
  }

  /// Load details for a specific item
  Future<void> loadItemDetails(String id) async {
    emit(state.copyWith(status: FormzSubmissionStatus.inProgress));

    try {
      // In a real implementation, you would fetch the specific item
      // For the template, we'll just use sample data
      final result = await _repository.getData();
      result.fold(
        (failure) {
          emit(state.copyWith(
            status: FormzSubmissionStatus.failure,
            errorMessage: failure.message,
          ));
        },
        (data) {
          emit(state.copyWith(
            status: FormzSubmissionStatus.success,
            selectedItem: data,
          ));
        },
      );
    } catch (e) {
      emit(state.copyWith(
        status: FormzSubmissionStatus.failure,
        errorMessage: e.toString(),
      ));
    }
  }

  /// Create a new item
  Future<void> createItem({required String title, required String description}) async {
    emit(state.copyWith(status: FormzSubmissionStatus.inProgress));

    try {
      final result = await _repository.createAuth(
        title: title,
        description: description,
      );

      result.fold(
        (failure) {
          emit(state.copyWith(
            status: FormzSubmissionStatus.failure,
            errorMessage: failure.message,
          ));
        },
        (item) {
          final updatedItems = [...state.items, item];
          emit(state.copyWith(
            status: FormzSubmissionStatus.success,
            items: updatedItems,
            successMessage: 'auth.item_created_success'.tr(),
          ));
        },
      );
    } catch (e) {
      emit(state.copyWith(
        status: FormzSubmissionStatus.failure,
        errorMessage: e.toString(),
      ));
    }
  }

  /// Update an existing item
  Future<void> updateItem({
    required String id,
    required String title,
    required String description,
  }) async {
    emit(state.copyWith(status: FormzSubmissionStatus.inProgress));

    try {
      final result = await _repository.updateAuth(
        id: id,
        title: title,
        description: description,
      );

      result.fold(
        (failure) {
          emit(state.copyWith(
            status: FormzSubmissionStatus.failure,
            errorMessage: failure.message,
          ));
        },
        (updatedItem) {
          final updatedItems = state.items.map((item) {
            return item.id == id ? updatedItem : item;
          }).toList();

          emit(state.copyWith(
            status: FormzSubmissionStatus.success,
            items: updatedItems,
            selectedItem: updatedItem,
            successMessage: 'auth.item_updated_success'.tr(),
          ));
        },
      );
    } catch (e) {
      emit(state.copyWith(
        status: FormzSubmissionStatus.failure,
        errorMessage: e.toString(),
      ));
    }
  }

  /// Delete an item
  Future<void> deleteItem(String id) async {
    emit(state.copyWith(status: FormzSubmissionStatus.inProgress));

    try {
      final result = await _repository.deleteAuth(id);

      result.fold(
        (failure) {
          emit(state.copyWith(
            status: FormzSubmissionStatus.failure,
            errorMessage: failure.message,
          ));
        },
        (_) {
          final filteredItems = state.items.where((item) => item.id != id).toList();
          emit(state.copyWith(
            status: FormzSubmissionStatus.success,
            items: filteredItems,
            selectedItem: null,
            successMessage: 'auth.item_deleted_success'.tr(),
          ));
        },
      );
    } catch (e) {
      emit(state.copyWith(
        status: FormzSubmissionStatus.failure,
        errorMessage: e.toString(),
      ));
    }
  }
  
  /// Start editing an item (prepare UI for editing)
  void editItem(AuthModel item) {
    emit(state.copyWith(
      editingItem: item,
    ));
  }
  
  /// Clear any messages to prevent showing them multiple times
  void clearMessages() {
    if (state.errorMessage != null || state.successMessage != null) {
      emit(state.copyWith(
        errorMessage: null,
        successMessage: null,
      ));
    }
  }
}
