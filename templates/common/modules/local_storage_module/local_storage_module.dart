import 'package:shared_preferences/shared_preferences.dart';

// Interface
abstract class LocalStorageModule {
  Future<bool> saveString(String key, String value);
  Future<bool> saveBool(String key, bool value);
  Future<bool> saveInt(String key, int value);
  Future<bool> saveDouble(String key, double value);
  Future<bool> saveStringList(String key, List<String> value);
  
  String? getString(String key);
  bool? getBool(String key);
  int? getInt(String key);
  double? getDouble(String key);
  List<String>? getStringList(String key);
  
  Future<bool> remove(String key);
  Future<bool> clear();
  bool containsKey(String key);
}

// Implementation using shared preferences
class LocalStorageImpl implements LocalStorageModule {
  final SharedPreferences _prefs;
  
  LocalStorageImpl._(this._prefs);
  
  static Future<LocalStorageImpl> init() async {
    final prefs = await SharedPreferences.getInstance();
    return LocalStorageImpl._(prefs);
  }
  
  @override
  Future<bool> saveString(String key, String value) {
    return _prefs.setString(key, value);
  }
  
  @override
  Future<bool> saveBool(String key, bool value) {
    return _prefs.setBool(key, value);
  }
  
  @override
  Future<bool> saveInt(String key, int value) {
    return _prefs.setInt(key, value);
  }
  
  @override
  Future<bool> saveDouble(String key, double value) {
    return _prefs.setDouble(key, value);
  }
  
  @override
  Future<bool> saveStringList(String key, List<String> value) {
    return _prefs.setStringList(key, value);
  }
  
  @override
  String? getString(String key) {
    return _prefs.getString(key);
  }
  
  @override
  bool? getBool(String key) {
    return _prefs.getBool(key);
  }
  
  @override
  int? getInt(String key) {
    return _prefs.getInt(key);
  }
  
  @override
  double? getDouble(String key) {
    return _prefs.getDouble(key);
  }
  
  @override
  List<String>? getStringList(String key) {
    return _prefs.getStringList(key);
  }
  
  @override
  Future<bool> remove(String key) {
    return _prefs.remove(key);
  }
  
  @override
  Future<bool> clear() {
    return _prefs.clear();
  }
  
  @override
  bool containsKey(String key) {
    return _prefs.containsKey(key);
  }
}
