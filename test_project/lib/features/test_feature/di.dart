import 'package:dio/dio.dart';
import 'package:get_it/get_it.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'cubits/test_feature_cubit/test_feature_cubit.dart';
import 'data/datasources/test_feature_local_datasource.dart';
import 'data/datasources/test_feature_remote_datasource.dart';
import 'data/repository/test_feature_repository.dart';

Future<void> initTestFeatureDi(GetIt getIt) async {
  // Register data sources
  getIt
    ..registerFactory<TestFeatureRemoteDatasource>(
      () => TestFeatureRemoteDatasourceImpl(
        dio: getIt<Dio>(),
      ),
    )
    ..registerFactory<TestFeatureLocalDatasource>(
      () => TestFeatureLocalDatasourceImpl(
        sharedPreferences: getIt<SharedPreferences>(),
      ),
    );
  
  // Register repositories
  getIt.registerFactory<TestFeatureRepository>(
    () => TestFeatureRepositoryImpl(),
  );
  
  // Register cubits
  getIt.registerFactory<TestFeatureCubit>(
    () => TestFeatureCubit(),
  );
}
