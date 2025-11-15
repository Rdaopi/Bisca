import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../services/auth_api.dart';
import '../services/game_api.dart';
import '../services/game_socket.dart';

final dioProvider = Provider<Dio>((_) {
  return Dio(
    BaseOptions(
      // Replace with the actual endpoint when available.
      baseUrl: 'http://localhost:3000',
      connectTimeout: const Duration(seconds: 5),
      receiveTimeout: const Duration(seconds: 10),
    ),
  );
});

final authApiProvider = Provider<AuthApi>((ref) {
  final Dio client = ref.watch(dioProvider);
  return AuthApi(client);
});

final gameApiProvider = Provider<GameApi>((ref) {
  final Dio client = ref.watch(dioProvider);
  return GameApi(client);
});

final gameSocketProvider = Provider<GameSocket>((_) {
  return GameSocket(Uri.parse('ws://localhost:3000/game'));
});
