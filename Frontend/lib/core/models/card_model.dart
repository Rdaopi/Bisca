class SuitParsingException implements FormatException {
  const SuitParsingException(this.message);
  @override
  final String message;
  @override
  final dynamic source = null;
  @override
  int? get offset => null;
}

enum Suit {
  denari,
  coppe,
  spade,
  bastoni;

  factory Suit.fromJson(String value) {
    switch (value) {
      case 'Denari':
        return Suit.denari;
      case 'Coppe':
        return Suit.coppe;
      case 'Spade':
        return Suit.spade;
      case 'Bastoni':
        return Suit.bastoni;
      default:
        throw SuitParsingException('Unknown suit: $value');
    }
  }

  String toJson() {
    switch (this) {
      case Suit.denari:
        return 'Denari';
      case Suit.coppe:
        return 'Coppe';
      case Suit.spade:
        return 'Spade';
      case Suit.bastoni:
        return 'Bastoni';
    }
  }
}

class ValueParsingException implements FormatException {
  const ValueParsingException(this.message);
  @override
  final String message;
  @override
  final dynamic source = null;
  @override
  int? get offset => null;
}

enum CardValue {
  asso(11),
  re(10),
  cavallo(9),
  fante(8),
  sette(7),
  sei(6),
  cinque(5),
  quattro(4),
  tre(3),
  due(2);

  const CardValue(this.strength);

  final int strength;

  factory CardValue.fromJson(String value) {
    switch (value) {
      case 'Asso':
        return CardValue.asso;
      case 'Re':
        return CardValue.re;
      case 'Cavallo':
        return CardValue.cavallo;
      case 'Fante':
        return CardValue.fante;
      case 'Sette':
        return CardValue.sette;
      case 'Sei':
        return CardValue.sei;
      case 'Cinque':
        return CardValue.cinque;
      case 'Quattro':
        return CardValue.quattro;
      case 'Tre':
        return CardValue.tre;
      case 'Due':
        return CardValue.due;
      default:
        throw ValueParsingException('Unknown card value: $value');
    }
  }

  String toJson() {
    switch (this) {
      case CardValue.asso:
        return 'Asso';
      case CardValue.re:
        return 'Re';
      case CardValue.cavallo:
        return 'Cavallo';
      case CardValue.fante:
        return 'Fante';
      case CardValue.sette:
        return 'Sette';
      case CardValue.sei:
        return 'Sei';
      case CardValue.cinque:
        return 'Cinque';
      case CardValue.quattro:
        return 'Quattro';
      case CardValue.tre:
        return 'Tre';
      case CardValue.due:
        return 'Due';
    }
  }
}

class CardModel {
  const CardModel({required this.suit, required this.value});

  final Suit suit;
  final CardValue value;

  factory CardModel.fromJson(Map<String, dynamic> json) {
    return CardModel(
      suit: Suit.fromJson(json['suit'] as String),
      value: CardValue.fromJson(json['value'] as String),
    );
  }

  Map<String, dynamic> toJson() => <String, dynamic>{
    'suit': suit.toJson(),
    'value': value.toJson(),
  };
}
