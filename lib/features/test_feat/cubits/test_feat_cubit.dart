import 'dart:developer';

import 'package:easy_localization/easy_localization.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:formz/formz.dart';

import '../../../../di.dart';
import '../../data/models/test_feat_model.dart';
import '../../data/repository/test_feat_repository.dart';

part 'test_feat_state.dart';

class TestFeatCubit extends Cubit<TestFeatState> {
  TestFeatCubit() : super(const TestFeatState());

  final TestFeatRepository _repository = getIt<TestFeatRepository>();

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
      final result = await _repository.createTestFeat(
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
            successMessage: 'test_feat.item_created_success'.tr(),
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

  /// For backward compatibility with older code
  Future<void> load() async {
    await initialize();
  }
}
