import 'dart:developer';
import 'package:dartz/dartz.dart';
import 'package:dio/dio.dart';
import 'package:logger/logger.dart';
import 'package:sentry_flutter/sentry_flutter.dart';

import '../../../../di.dart';
import '../../../../helpers/error_helper.dart';
import '../../../../models/failures/failure.dart';
import '../datasources/test_feat_local_datasource.dart';
import '../datasources/test_feat_remote_datasource.dart';
import '../models/test_feat_model.dart';
import '../models/failure.dart';

abstract class TestFeatRepository {
  /// Gets data from the TestFeat feature
  ///
  /// Returns [TestFeatModel] if successful
  /// Returns [Failure] if an error occurs
  Future<Either<Failure, TestFeatModel>> getData();
  
  /// Creates a new TestFeat item
  ///
  /// Returns the created [TestFeatModel] if successful
  /// Returns [Failure] if an error occurs
  Future<Either<Failure, TestFeatModel>> createTestFeat({
    required String title,
    required String description,
  });
  
  /// Updates an existing TestFeat item
  ///
  /// Returns the updated [TestFeatModel] if successful
  /// Returns [TestFeatFailure] if an error occurs
  Future<Either<TestFeatFailure, TestFeatModel>> updateTestFeat({
    required String id,
    required String title,
    required String description,
  });
  
  /// Deletes a TestFeat item
  ///
  /// Returns void if successful
  /// Returns [Failure] if an error occurs
  Future<Either<Failure, void>> deleteTestFeat(String id);
}

class TestFeatRepositoryImpl implements TestFeatRepository {
  TestFeatRepositoryImpl();

  final TestFeatRemoteDatasource _remoteDatasource = getIt();
  final TestFeatLocalDatasource _localDatasource = getIt();
  final Logger logger = Logger();

  @override
  Future<Either<Failure, TestFeatModel>> getData() async {
    try {
      // First try to get data from local cache
      final localData = await _localDatasource.getTestFeatData();
      
      if (localData != null) {
        return Right(localData);
      }
      
      // If local data is not available, fetch from remote
      final remoteData = await _remoteDatasource.fetchTestFeatData();
      
      // Cache the remote data locally for future use
      await _localDatasource.saveTestFeatData(remoteData);
      
      return Right(remoteData);
    } catch (e, stackTrace) {
      log('🐞Error: $e', stackTrace: stackTrace);
      return Left(
        ErrorHelper.errorToFailure(e, stacktrace: stackTrace),
      );
    }
  }

  @override
  Future<Either<Failure, TestFeatModel>> createTestFeat({
    required String title,
    required String description,
  }) async {
    try {
      final result = await _remoteDatasource.createTestFeat({
        'title': title,
        'description': description,
      });
      
      // Update local cache with the new item
      await _localDatasource.saveTestFeatItem(result);
      
      return Right(result);
    } catch (e, stackTrace) {
      log('🐞Error: $e', stackTrace: stackTrace);
      return Left(
        ErrorHelper.errorToFailure(e, stacktrace: stackTrace),
      );
    }
  }

  @override
  Future<Either<TestFeatFailure, TestFeatModel>> updateTestFeat({
    required String id,
    required String title,
    required String description,
  }) async {
    try {
      final result = await _remoteDatasource.updateTestFeat(
        id: id,
        data: {
          'title': title,
          'description': description,
        },
      );
      
      // Update local cache with the updated item
      await _localDatasource.updateTestFeatItem(result);
      
      return Right(result);
    } on DioException catch (e) {
      logger.e('DioException: ${e.message}');
      final resData = e.response?.data as Map<String, dynamic>?;
      final error = resData == null ? null : Failure.fromJson(resData);
      
      if (e.response?.statusCode == 404) {
        return Left(
          TestFeatFailure(
            message: error?.message ?? 'Item not found',
            code: e.response?.statusCode ?? 404,
            result: TestFeatResult.notFound,
          ),
        );
      }
      
      return Left(
        TestFeatFailure(
          message: error?.message ?? 'Failed to update item',
          code: e.response?.statusCode ?? 500,
          result: TestFeatResult.failure,
        ),
      );
    } catch (e, stackTrace) {
      logger.e('Error: $e', stackTrace: stackTrace);
      await Sentry.captureException(
        e,
        stackTrace: stackTrace,
      );
      
      return Left(
        TestFeatFailure(
          message: e.toString(),
          result: TestFeatResult.failure,
        ),
      );
    }
  }

  @override
  Future<Either<Failure, void>> deleteTestFeat(String id) async {
    try {
      await _remoteDatasource.deleteTestFeat(id);
      
      // Remove item from local cache
      await _localDatasource.deleteTestFeatItem(id);
      
      return const Right(null);
    } catch (e, stackTrace) {
      log('🐞Error: $e', stackTrace: stackTrace);
      return Left(
        ErrorHelper.errorToFailure(e, stacktrace: stackTrace),
      );
    }
  }
}
