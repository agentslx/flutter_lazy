import 'package:dio/dio.dart';
import 'package:get_it/get_it.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'cubits/test_feat_cubit/test_feat_cubit.dart';
import 'data/datasources/test_feat_local_datasource.dart';
import 'data/datasources/test_feat_remote_datasource.dart';
import 'data/repository/test_feat_repository.dart';

Future<void> initTestFeatDi(GetIt getIt) async {
  // Register data sources
  getIt
    ..registerFactory<TestFeatRemoteDatasource>(
      () => TestFeatRemoteDatasourceImpl(
        dio: getIt<Dio>(),
      ),
    )
    ..registerFactory<TestFeatLocalDatasource>(
      () => TestFeatLocalDatasourceImpl(
        sharedPreferences: getIt<SharedPreferences>(),
      ),
    );
  
  // Register repositories
  getIt.registerFactory<TestFeatRepository>(
    () => TestFeatRepositoryImpl(),
  );
  
  // Register cubits
  getIt.registerFactory<TestFeatCubit>(
    () => TestFeatCubit(),
  );
}
