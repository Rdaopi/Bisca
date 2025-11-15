import 'card_model.dart';

enum GameEventType {
  welcome,
  playerJoined,
  playerLeft,
  handUpdated,
  cardPlayed,
  predictionMade,
  roundStarted,
  roundEnded,
  turnEnded,
  gameStarted,
  gameOver,
  error,
  unknown;

  factory GameEventType.fromString(String value) {
    switch (value) {
      case 'welcome':
        return GameEventType.welcome;
      case 'player_joined':
        return GameEventType.playerJoined;
      case 'player_left':
        return GameEventType.playerLeft;
      case 'hand_updated':
        return GameEventType.handUpdated;
      case 'card_played':
        return GameEventType.cardPlayed;
      case 'prediction_made':
        return GameEventType.predictionMade;
      case 'round_started':
        return GameEventType.roundStarted;
      case 'round_ended':
        return GameEventType.roundEnded;
      case 'turn_ended':
        return GameEventType.turnEnded;
      case 'game_started':
        return GameEventType.gameStarted;
      case 'game_over':
        return GameEventType.gameOver;
      case 'error':
        return GameEventType.error;
      default:
        return GameEventType.unknown;
    }
  }
}

class GameEvent {
  const GameEvent({required this.type, required this.data});

  final GameEventType type;
  final Map<String, dynamic> data;

  factory GameEvent.fromJson(Map<String, dynamic> json) {
    final String typeValue = json['event'] as String? ?? 'unknown';
    final Map<String, dynamic> payload =
        (json['data'] as Map<String, dynamic>?) ?? <String, dynamic>{};
    return GameEvent(type: GameEventType.fromString(typeValue), data: payload);
  }

  List<CardModel> parseHand() {
    final List<dynamic> raw = (data['hand'] as List<dynamic>?) ?? <dynamic>[];
    return raw
        .map((dynamic item) => CardModel.fromJson(item as Map<String, dynamic>))
        .toList();
  }
}
