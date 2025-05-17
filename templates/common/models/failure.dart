import 'package:equatable/equatable.dart';

class Failure extends Equatable {
  final String message;
  final int? code;
  final dynamic data;

  const Failure(this.message, {this.code, this.data});

  @override
  List<Object?> get props => [message, code, data];
}
