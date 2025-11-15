import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/models/card_model.dart';
import '../controllers/game_controller.dart';

class GameScreen extends ConsumerWidget {
  const GameScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final GameViewState state = ref.watch(gameControllerProvider);
    final GameController controller = ref.read(gameControllerProvider.notifier);

    return Scaffold(
      appBar: AppBar(
        title: Text('Round ${state.roundNumber} / ${state.startingCards}'),
        actions: <Widget>[
          IconButton(
            onPressed: controller.startGame,
            icon: const Icon(Icons.play_arrow),
            tooltip: 'Avvia partita',
          ),
          IconButton(
            onPressed: controller.endTurn,
            icon: const Icon(Icons.flag),
            tooltip: 'Chiudi turno',
          ),
        ],
      ),
      body: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: <Widget>[
          if (state.message != null)
            Padding(
              padding: const EdgeInsets.all(16),
              child: Text(
                state.message!,
                style: const TextStyle(color: Colors.red),
              ),
            ),
          Padding(
            padding: const EdgeInsets.all(16),
            child: Text(
              'Carte giocate',
              style: Theme.of(context).textTheme.titleMedium,
            ),
          ),
          SizedBox(
            height: 120,
            child: ListView.builder(
              scrollDirection: Axis.horizontal,
              itemCount: state.currentTurnCards.length,
              itemBuilder: (BuildContext context, int index) {
                final Map<String, dynamic> item = state.currentTurnCards[index];
                final Map<String, dynamic>? card =
                    item['card'] as Map<String, dynamic>?;
                return Card(
                  child: Padding(
                    padding: const EdgeInsets.all(12),
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: <Widget>[
                        Text(item['player_id'] as String? ?? '???'),
                        if (card != null)
                          Text('${card['value']} di ${card['suit']}'),
                      ],
                    ),
                  ),
                );
              },
            ),
          ),
          const Divider(),
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            child: Text(
              'La tua mano',
              style: Theme.of(context).textTheme.titleMedium,
            ),
          ),
          Expanded(
            child: GridView.builder(
              padding: const EdgeInsets.all(16),
              gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
                crossAxisCount: 3,
                childAspectRatio: 3 / 4,
                crossAxisSpacing: 12,
                mainAxisSpacing: 12,
              ),
              itemCount: state.hand.length,
              itemBuilder: (BuildContext context, int index) {
                final CardModel card = state.hand[index];
                return GestureDetector(
                  onTap: () => controller.playCard(card),
                  child: Card(
                    child: Center(
                      child: Text(
                        '${card.value.toJson()} di ${card.suit.toJson()}',
                      ),
                    ),
                  ),
                );
              },
            ),
          ),
          Padding(
            padding: const EdgeInsets.all(16),
            child: ElevatedButton(
              onPressed: () => controller.nextRound(),
              child: const Text('Prossimo round'),
            ),
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () => controller.makePrediction(0),
        icon: const Icon(Icons.analytics),
        label: const Text('Fai previsione'),
      ),
    );
  }
}
