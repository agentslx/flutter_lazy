import 'package:dio/dio.dart';
import 'package:get_it/get_it.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'cubits/auth_cubit/auth_cubit.dart';
import 'data/datasources/auth_local_datasource.dart';
import 'data/datasources/auth_remote_datasource.dart';
import 'data/repository/auth_repository.dart';

Future<void> initAuthDi(GetIt getIt) async {
  // Register data sources
  getIt
    ..registerFactory<AuthRemoteDatasource>(
      () => AuthRemoteDatasourceImpl(
        dio: getIt<Dio>(),
      ),
    )
    ..registerFactory<AuthLocalDatasource>(
      () => AuthLocalDatasourceImpl(
        sharedPreferences: getIt<SharedPreferences>(),
      ),
    );
  
  // Register repositories
  getIt.registerFactory<AuthRepository>(
    () => AuthRepositoryImpl(),
  );
  
  // Register cubits
  getIt.registerFactory<AuthCubit>(
    () => AuthCubit(),
  );
}
