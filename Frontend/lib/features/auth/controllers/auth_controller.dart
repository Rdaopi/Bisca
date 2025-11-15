import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/services/auth_api.dart';
import '../../../core/state/app_providers.dart';

enum AuthStatus { unknown, loading, authenticated, unauthenticated, error }

class AuthState {
  const AuthState({this.status = AuthStatus.unknown, this.token, this.message});

  final AuthStatus status;
  final String? token;
  final String? message;

  AuthState copyWith({AuthStatus? status, String? token, String? message}) {
    return AuthState(
      status: status ?? this.status,
      token: token ?? this.token,
      message: message ?? this.message,
    );
  }
}

class AuthController extends StateNotifier<AuthState> {
  AuthController(this._authApi) : super(const AuthState());

  final AuthApi _authApi;

  Future<void> login(String email, String password) async {
    state = state.copyWith(status: AuthStatus.loading, message: null);
    try {
      final response = await _authApi.login(email: email, password: password);
      final String token = response.data['token'] as String? ?? '';
      state = state.copyWith(status: AuthStatus.authenticated, token: token);
    } catch (err) {
      state = state.copyWith(status: AuthStatus.error, message: err.toString());
    }
  }

  Future<void> register(String email, String password, String username) async {
    state = state.copyWith(status: AuthStatus.loading, message: null);
    try {
      await _authApi.register(
        email: email,
        password: password,
        username: username,
      );
      state = state.copyWith(status: AuthStatus.unauthenticated);
    } catch (err) {
      state = state.copyWith(status: AuthStatus.error, message: err.toString());
    }
  }

  void logout() {
    state = const AuthState(status: AuthStatus.unauthenticated);
  }
}

final authControllerProvider = StateNotifierProvider<AuthController, AuthState>(
  (ref) {
    final AuthApi api = ref.watch(authApiProvider);
    return AuthController(api);
  },
);
