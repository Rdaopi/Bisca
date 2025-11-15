import 'package:dio/dio.dart';

class AuthApi {
  AuthApi(this._client);

  final Dio _client;

  Future<Response<dynamic>> login({
    required String email,
    required String password,
  }) {
    return _client.post<dynamic>(
      '/login',
      data: <String, dynamic>{'email': email, 'password': password},
    );
  }

  Future<Response<dynamic>> register({
    required String email,
    required String password,
    required String username,
  }) {
    return _client.post<dynamic>(
      '/register',
      data: <String, dynamic>{
        'email': email,
        'password': password,
        'username': username,
      },
    );
  }
}
