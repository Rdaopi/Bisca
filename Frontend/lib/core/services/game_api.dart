import 'package:dio/dio.dart';

class GameApi {
  GameApi(this._client);

  final Dio _client;

  Future<Response<dynamic>> createGame() {
    return _client.post<dynamic>('/games');
  }

  Future<Response<dynamic>> joinGame(String gameId) {
    return _client.post<dynamic>('/games/$gameId/join');
  }

  Future<Response<dynamic>> playCard(String gameId, Map<String, dynamic> card) {
    return _client.post<dynamic>(
      '/games/$gameId/play-card',
      data: <String, dynamic>{'card': card},
    );
  }

  Future<Response<dynamic>> makePrediction(String gameId, int prediction) {
    return _client.post<dynamic>(
      '/games/$gameId/prediction',
      data: <String, dynamic>{'prediction': prediction},
    );
  }
}
