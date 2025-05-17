import 'package:get_it/get_it.dart';

// Core modules
import 'modules/local_storage_module/local_storage_module.dart';
import 'modules/rest_module/rest_module.dart';
import 'modules/push_notification/push_notification_module.dart';

// Features
import 'features/auth/di.dart' as auth_di;
import 'features/notification/di.dart' as notification_di;
import 'features/home/di.dart' as home_di;

final getIt = GetIt.instance;

Future<void> initDependencies() async {
  // Initialize core modules
  await _initCoreModules();
  
  // Initialize features
  await _initFeatures();
}

Future<void> _initCoreModules() async {
  // Local storage
  final localStorageModule = await LocalStorageImpl.init();
  getIt.registerSingleton<LocalStorageModule>(localStorageModule);
  
  // Network module
  final networkModule = NetworkModuleImpl();
  getIt.registerSingleton<NetworkModule>(networkModule);
  
  // Push notification module
  final pushNotificationModule = PushNotificationModuleImpl();
  getIt.registerSingleton<PushNotificationModule>(pushNotificationModule);
}

Future<void> _initFeatures() async {
  await auth_di.init(getIt);
  await notification_di.init(getIt);
  await home_di.init(getIt);
}
