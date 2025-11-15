import 'dart:async';
import 'dart:convert';

import 'package:web_socket_channel/web_socket_channel.dart';

import '../models/game_event.dart';

class GameSocket {
  GameSocket(this.uri);

  final Uri uri;
  WebSocketChannel? _channel;
  StreamSubscription<dynamic>? _subscription;
  final StreamController<GameEvent> _events =
      StreamController<GameEvent>.broadcast();

  Stream<GameEvent> get events => _events.stream;

  Future<void> connect() async {
    _channel = WebSocketChannel.connect(uri);
    _subscription = _channel!.stream.listen(
      (dynamic data) {
        if (data is String) {
          final Map<String, dynamic> decoded =
              jsonDecode(data) as Map<String, dynamic>;
          _events.add(GameEvent.fromJson(decoded));
        }
      },
      onError: (Object err) {
        _events.add(
          GameEvent(
            type: GameEventType.error,
            data: <String, dynamic>{'message': err.toString()},
          ),
        );
      },
      onDone: () {
        _events.add(
          const GameEvent(
            type: GameEventType.error,
            data: <String, dynamic>{'message': 'Connection closed'},
          ),
        );
      },
      cancelOnError: true,
    );
  }

  void sendAction(String action, Map<String, dynamic> payload) {
    final Map<String, dynamic> data = <String, dynamic>{
      'action': action,
      ...payload,
    };
    _channel?.sink.add(jsonEncode(data));
  }

  Future<void> dispose() async {
    await _subscription?.cancel();
    await _channel?.sink.close();
    await _events.close();
  }
}
