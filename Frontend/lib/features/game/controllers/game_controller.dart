import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/models/card_model.dart';
import '../../../core/models/game_event.dart';
import '../../../core/models/player_model.dart';
import '../../../core/services/game_socket.dart';
import '../../../core/state/app_providers.dart';

enum GameStatus { initial, connecting, connected, disconnected, error }

class GameViewState {
  const GameViewState({
    this.status = GameStatus.initial,
    this.hand = const <CardModel>[],
    this.players = const <PlayerModel>[],
    this.roundNumber = 1,
    this.startingCards = 0,
    this.currentTurnCards = const <Map<String, dynamic>>[],
    this.message,
    this.playerId,
  });

  final GameStatus status;
  final List<CardModel> hand;
  final List<PlayerModel> players;
  final int roundNumber;
  final int startingCards;
  final List<Map<String, dynamic>> currentTurnCards;
  final String? message;
  final String? playerId;

  GameViewState copyWith({
    GameStatus? status,
    List<CardModel>? hand,
    List<PlayerModel>? players,
    int? roundNumber,
    int? startingCards,
    List<Map<String, dynamic>>? currentTurnCards,
    String? message,
    String? playerId,
  }) {
    return GameViewState(
      status: status ?? this.status,
      hand: hand ?? this.hand,
      players: players ?? this.players,
      roundNumber: roundNumber ?? this.roundNumber,
      startingCards: startingCards ?? this.startingCards,
      currentTurnCards: currentTurnCards ?? this.currentTurnCards,
      message: message,
      playerId: playerId ?? this.playerId,
    );
  }
}

class GameController extends StateNotifier<GameViewState> {
  GameController(this._socket) : super(const GameViewState());

  final GameSocket _socket;
  StreamSubscription<GameEvent>? _subscription;

  Future<void> connect() async {
    state = state.copyWith(status: GameStatus.connecting, message: null);
    await _socket.connect();
    _subscription = _socket.events.listen(_onEvent);
  }

  void _onEvent(GameEvent event) {
    switch (event.type) {
      case GameEventType.welcome:
        state = state.copyWith(
          status: GameStatus.connected,
          hand: (event.data['hand'] as List<dynamic>? ?? <dynamic>[])
              .map((dynamic e) => CardModel.fromJson(e as Map<String, dynamic>))
              .toList(),
          players: (event.data['players'] as List<dynamic>? ?? <dynamic>[])
              .map(
                (dynamic e) => PlayerModel(id: e as String, isConnected: true),
              )
              .toList(),
          roundNumber: event.data['round_number'] as int? ?? 1,
          startingCards: event.data['starting_cards'] as int? ?? 0,
          currentTurnCards:
              (event.data['turn'] as List<dynamic>? ?? <dynamic>[])
                  .cast<Map<String, dynamic>>(),
          playerId: event.data['player_id'] as String?,
        );
        break;
      case GameEventType.handUpdated:
        state = state.copyWith(hand: event.parseHand());
        break;
      case GameEventType.cardPlayed:
        state = state.copyWith(
          currentTurnCards: <Map<String, dynamic>>[
            ...state.currentTurnCards,
            event.data,
          ],
        );
        break;
      case GameEventType.playerJoined:
        state = state.copyWith(
          players: <PlayerModel>[
            ...state.players,
            PlayerModel(id: event.data['id'] as String? ?? 'unknown'),
          ],
        );
        break;
      case GameEventType.playerLeft:
        state = state.copyWith(
          players: state.players
              .map(
                (PlayerModel player) => player.id == event.data['id']
                    ? player.copyWith(isConnected: false)
                    : player,
              )
              .toList(),
        );
        break;
      case GameEventType.turnEnded:
        state = state.copyWith(currentTurnCards: <Map<String, dynamic>>[]);
        break;
      case GameEventType.roundStarted:
        state = state.copyWith(
          roundNumber: event.data['round_number'] as int? ?? state.roundNumber,
          startingCards:
              event.data['starting_cards'] as int? ?? state.startingCards,
        );
        break;
      case GameEventType.roundEnded:
      case GameEventType.gameOver:
      case GameEventType.gameStarted:
      case GameEventType.predictionMade:
        // Detailed handling will be implemented later.
        break;
      case GameEventType.error:
        state = state.copyWith(
          status: GameStatus.error,
          message: event.data['message'] as String?,
        );
        break;
      case GameEventType.unknown:
        break;
    }
  }

  void startGame() {
    _socket.sendAction('start_game', <String, dynamic>{});
  }

  void playCard(CardModel card) {
    _socket.sendAction('play_card', <String, dynamic>{'card': card.toJson()});
  }

  void makePrediction(int value) {
    _socket.sendAction('make_prediction', <String, dynamic>{
      'prediction': value,
    });
  }

  void endTurn() {
    _socket.sendAction('end_turn', <String, dynamic>{});
  }

  void nextRound() {
    _socket.sendAction('next_round', <String, dynamic>{});
  }

  @override
  void dispose() {
    final Future<void>? cancelFuture = _subscription?.cancel();
    if (cancelFuture != null) {
      unawaited(cancelFuture);
    }
    unawaited(_socket.dispose());
    super.dispose();
  }
}

final gameControllerProvider =
    StateNotifierProvider<GameController, GameViewState>((ref) {
      final GameSocket socket = ref.watch(gameSocketProvider);
      final controller = GameController(socket);
      controller.connect();
      return controller;
    });
