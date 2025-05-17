import 'dart:developer';
import 'package:dartz/dartz.dart';
import 'package:dio/dio.dart';
import 'package:logger/logger.dart';
import 'package:sentry_flutter/sentry_flutter.dart';

import '../../../../di.dart';
import '../../../../helpers/error_helper.dart';
import '../../../../models/failures/failure.dart';
import '../datasources/test_feature_local_datasource.dart';
import '../datasources/test_feature_remote_datasource.dart';
import '../models/test_feature_model.dart';
import '../models/failure.dart';

abstract class TestFeatureRepository {
  /// Gets data from the TestFeature feature
  ///
  /// Returns [TestFeatureModel] if successful
  /// Returns [Failure] if an error occurs
  Future<Either<Failure, TestFeatureModel>> getData();
  
  /// Creates a new TestFeature item
  ///
  /// Returns the created [TestFeatureModel] if successful
  /// Returns [Failure] if an error occurs
  Future<Either<Failure, TestFeatureModel>> createTestFeature({
    required String title,
    required String description,
  });
  
  /// Updates an existing TestFeature item
  ///
  /// Returns the updated [TestFeatureModel] if successful
  /// Returns [TestFeatureFailure] if an error occurs
  Future<Either<TestFeatureFailure, TestFeatureModel>> updateTestFeature({
    required String id,
    required String title,
    required String description,
  });
  
  /// Deletes a TestFeature item
  ///
  /// Returns void if successful
  /// Returns [Failure] if an error occurs
  Future<Either<Failure, void>> deleteTestFeature(String id);
}

class TestFeatureRepositoryImpl implements TestFeatureRepository {
  TestFeatureRepositoryImpl();

  final TestFeatureRemoteDatasource _remoteDatasource = getIt();
  final TestFeatureLocalDatasource _localDatasource = getIt();
  final Logger logger = Logger();

  @override
  Future<Either<Failure, TestFeatureModel>> getData() async {
    try {
      // First try to get data from local cache
      final localData = await _localDatasource.getTestFeatureData();
      
      if (localData != null) {
        return Right(localData);
      }
      
      // If local data is not available, fetch from remote
      final remoteData = await _remoteDatasource.fetchTestFeatureData();
      
      // Cache the remote data locally for future use
      await _localDatasource.saveTestFeatureData(remoteData);
      
      return Right(remoteData);
    } catch (e, stackTrace) {
      log('🐞Error: $e', stackTrace: stackTrace);
      return Left(
        ErrorHelper.errorToFailure(e, stacktrace: stackTrace),
      );
    }
  }

  @override
  Future<Either<Failure, TestFeatureModel>> createTestFeature({
    required String title,
    required String description,
  }) async {
    try {
      final result = await _remoteDatasource.createTestFeature({
        'title': title,
        'description': description,
      });
      
      // Update local cache with the new item
      await _localDatasource.saveTestFeatureItem(result);
      
      return Right(result);
    } catch (e, stackTrace) {
      log('🐞Error: $e', stackTrace: stackTrace);
      return Left(
        ErrorHelper.errorToFailure(e, stacktrace: stackTrace),
      );
    }
  }

  @override
  Future<Either<TestFeatureFailure, TestFeatureModel>> updateTestFeature({
    required String id,
    required String title,
    required String description,
  }) async {
    try {
      final result = await _remoteDatasource.updateTestFeature(
        id: id,
        data: {
          'title': title,
          'description': description,
        },
      );
      
      // Update local cache with the updated item
      await _localDatasource.updateTestFeatureItem(result);
      
      return Right(result);
    } on DioException catch (e) {
      logger.e('DioException: ${e.message}');
      final resData = e.response?.data as Map<String, dynamic>?;
      final error = resData == null ? null : Failure.fromJson(resData);
      
      if (e.response?.statusCode == 404) {
        return Left(
          TestFeatureFailure(
            message: error?.message ?? 'Item not found',
            code: e.response?.statusCode ?? 404,
            result: TestFeatureResult.notFound,
          ),
        );
      }
      
      return Left(
        TestFeatureFailure(
          message: error?.message ?? 'Failed to update item',
          code: e.response?.statusCode ?? 500,
          result: TestFeatureResult.failure,
        ),
      );
    } catch (e, stackTrace) {
      logger.e('Error: $e', stackTrace: stackTrace);
      await Sentry.captureException(
        e,
        stackTrace: stackTrace,
      );
      
      return Left(
        TestFeatureFailure(
          message: e.toString(),
          result: TestFeatureResult.failure,
        ),
      );
    }
  }

  @override
  Future<Either<Failure, void>> deleteTestFeature(String id) async {
    try {
      await _remoteDatasource.deleteTestFeature(id);
      
      // Remove item from local cache
      await _localDatasource.deleteTestFeatureItem(id);
      
      return const Right(null);
    } catch (e, stackTrace) {
      log('🐞Error: $e', stackTrace: stackTrace);
      return Left(
        ErrorHelper.errorToFailure(e, stacktrace: stackTrace),
      );
    }
  }
}
