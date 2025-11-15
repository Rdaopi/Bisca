import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/models/player_model.dart';

class LobbyState {
  const LobbyState({
    this.players = const <PlayerModel>[],
    this.gameId,
    this.isLoading = false,
    this.errorMessage,
  });

  final List<PlayerModel> players;
  final String? gameId;
  final bool isLoading;
  final String? errorMessage;

  LobbyState copyWith({
    List<PlayerModel>? players,
    String? gameId,
    bool? isLoading,
    String? errorMessage,
  }) {
    return LobbyState(
      players: players ?? this.players,
      gameId: gameId ?? this.gameId,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage,
    );
  }
}

class LobbyController extends StateNotifier<LobbyState> {
  LobbyController() : super(const LobbyState());

  void setPlayers(List<PlayerModel> players) {
    state = state.copyWith(players: players);
  }

  void setGameId(String id) {
    state = state.copyWith(gameId: id);
  }

  void setLoading(bool value) {
    state = state.copyWith(isLoading: value);
  }

  void setError(String? message) {
    state = state.copyWith(errorMessage: message);
  }
}

final lobbyControllerProvider =
    StateNotifierProvider<LobbyController, LobbyState>((_) {
      return LobbyController();
    });
