import 'package:dio/dio.dart';
import 'package:pretty_dio_logger/pretty_dio_logger.dart';
import 'package:dartz/dartz.dart';

import '../../models/failure.dart';
import '../../flavors.dart';

// Interface
abstract class NetworkModule {
  Future<Either<Failure, T>> get<T>(
    String path, {
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    T Function(dynamic)? parser,
  });
  
  Future<Either<Failure, T>> post<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    T Function(dynamic)? parser,
  });
  
  Future<Either<Failure, T>> put<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    T Function(dynamic)? parser,
  });
  
  Future<Either<Failure, T>> delete<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    T Function(dynamic)? parser,
  });
  
  void setAuthToken(String token);
  void removeAuthToken();
}

// Implementation using Dio
class NetworkModuleImpl implements NetworkModule {
  late final Dio _dio;
  
  NetworkModuleImpl() {
    _dio = Dio(
      BaseOptions(
        baseUrl: F.baseUrl,
        connectTimeout: const Duration(seconds: 30),
        receiveTimeout: const Duration(seconds: 30),
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
        },
      ),
    );
    
    // Add logging in debug mode
    if (F.isDev) {
      _dio.interceptors.add(
        PrettyDioLogger(
          requestHeader: true,
          requestBody: true,
          responseBody: true,
          responseHeader: false,
          error: true,
          compact: true,
          maxWidth: 90,
        ),
      );
    }
  }
  
  @override
  Future<Either<Failure, T>> get<T>(
    String path, {
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    T Function(dynamic)? parser,
  }) async {
    try {
      final response = await _dio.get(
        path,
        queryParameters: queryParameters,
        options: options,
        cancelToken: cancelToken,
      );
      
      return Right(parser != null ? parser(response.data) : response.data as T);
    } on DioException catch (e) {
      return Left(_handleDioError(e));
    } catch (e) {
      return Left(Failure('Something went wrong: $e'));
    }
  }
  
  @override
  Future<Either<Failure, T>> post<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    T Function(dynamic)? parser,
  }) async {
    try {
      final response = await _dio.post(
        path,
        data: data,
        queryParameters: queryParameters,
        options: options,
        cancelToken: cancelToken,
      );
      
      return Right(parser != null ? parser(response.data) : response.data as T);
    } on DioException catch (e) {
      return Left(_handleDioError(e));
    } catch (e) {
      return Left(Failure('Something went wrong: $e'));
    }
  }
  
  @override
  Future<Either<Failure, T>> put<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    T Function(dynamic)? parser,
  }) async {
    try {
      final response = await _dio.put(
        path,
        data: data,
        queryParameters: queryParameters,
        options: options,
        cancelToken: cancelToken,
      );
      
      return Right(parser != null ? parser(response.data) : response.data as T);
    } on DioException catch (e) {
      return Left(_handleDioError(e));
    } catch (e) {
      return Left(Failure('Something went wrong: $e'));
    }
  }
  
  @override
  Future<Either<Failure, T>> delete<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    T Function(dynamic)? parser,
  }) async {
    try {
      final response = await _dio.delete(
        path,
        data: data,
        queryParameters: queryParameters,
        options: options,
        cancelToken: cancelToken,
      );
      
      return Right(parser != null ? parser(response.data) : response.data as T);
    } on DioException catch (e) {
      return Left(_handleDioError(e));
    } catch (e) {
      return Left(Failure('Something went wrong: $e'));
    }
  }
  
  @override
  void setAuthToken(String token) {
    _dio.options.headers['Authorization'] = 'Bearer $token';
  }
  
  @override
  void removeAuthToken() {
    _dio.options.headers.remove('Authorization');
  }
  
  Failure _handleDioError(DioException error) {
    switch (error.type) {
      case DioExceptionType.connectionTimeout:
      case DioExceptionType.sendTimeout:
      case DioExceptionType.receiveTimeout:
        return Failure('Connection timeout', code: error.response?.statusCode);
      
      case DioExceptionType.cancel:
        return Failure('Request canceled', code: error.response?.statusCode);
      
      case DioExceptionType.badResponse:
        return _handleErrorResponse(error.response);
      
      case DioExceptionType.connectionError:
        return Failure('Connection error', code: error.response?.statusCode);
      
      default:
        return Failure('Network error: ${error.message}', code: error.response?.statusCode);
    }
  }
  
  Failure _handleErrorResponse(Response? response) {
    if (response == null) {
      return const Failure('No response from server');
    }
    
    final statusCode = response.statusCode;
    final data = response.data;
    
    // Try to extract error message from response
    String errorMessage = 'Unknown error occurred';
    
    if (data != null && data is Map<String, dynamic>) {
      if (data.containsKey('message')) {
        errorMessage = data['message'].toString();
      } else if (data.containsKey('error')) {
        errorMessage = data['error'].toString();
      }
    }
    
    return Failure(errorMessage, code: statusCode, data: data);
  }
}
