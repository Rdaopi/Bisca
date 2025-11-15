import 'card_model.dart';

class PlayerModel {
  const PlayerModel({
    required this.id,
    this.hand = const <CardModel>[],
    this.prediction,
    this.tricksWon = 0,
    this.isConnected = true,
  });

  final String id;
  final List<CardModel> hand;
  final int? prediction;
  final int tricksWon;
  final bool isConnected;

  factory PlayerModel.fromJson(Map<String, dynamic> json) {
    return PlayerModel(
      id: json['id'] as String,
      hand: (json['hand'] as List<dynamic>? ?? <dynamic>[])
          .map(
            (dynamic item) => CardModel.fromJson(item as Map<String, dynamic>),
          )
          .toList(),
      prediction: json['prediction'] as int?,
      tricksWon: json['tricks_won'] as int? ?? 0,
      isConnected: json['is_connected'] as bool? ?? true,
    );
  }

  Map<String, dynamic> toJson() => <String, dynamic>{
    'id': id,
    'hand': hand.map((CardModel card) => card.toJson()).toList(),
    'prediction': prediction,
    'tricks_won': tricksWon,
    'is_connected': isConnected,
  };

  PlayerModel copyWith({
    String? id,
    List<CardModel>? hand,
    int? prediction,
    int? tricksWon,
    bool? isConnected,
  }) {
    return PlayerModel(
      id: id ?? this.id,
      hand: hand ?? this.hand,
      prediction: prediction ?? this.prediction,
      tricksWon: tricksWon ?? this.tricksWon,
      isConnected: isConnected ?? this.isConnected,
    );
  }
}
