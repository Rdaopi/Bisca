import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../routes/app_router.dart';
import '../controllers/lobby_controller.dart';

class LobbyScreen extends ConsumerWidget {
  const LobbyScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final LobbyState state = ref.watch(lobbyControllerProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Lobby'),
        actions: <Widget>[
          IconButton(
            onPressed: () {
              Navigator.of(context).pushNamed(AppRouter.game);
            },
            icon: const Icon(Icons.play_arrow),
            tooltip: 'Avvia partita',
          ),
        ],
      ),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: <Widget>[
            Text(
              'Codice partita: ${state.gameId ?? 'non disponibile'}',
              style: Theme.of(context).textTheme.titleMedium,
            ),
            const SizedBox(height: 16),
            Text(
              'Giocatori (${state.players.length}):',
              style: Theme.of(context).textTheme.titleSmall,
            ),
            const SizedBox(height: 8),
            Expanded(
              child: ListView.builder(
                itemCount: state.players.length,
                itemBuilder: (BuildContext context, int index) {
                  final player = state.players[index];
                  return ListTile(
                    title: Text(player.id),
                    subtitle: Text(
                      player.isConnected ? 'Connesso' : 'Disconnesso',
                    ),
                  );
                },
              ),
            ),
            if (state.errorMessage != null)
              Text(
                state.errorMessage!,
                style: const TextStyle(color: Colors.red),
              ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () {
          Navigator.of(context).pushNamed(AppRouter.game);
        },
        icon: const Icon(Icons.sports_esports),
        label: const Text('Entra in gioco'),
      ),
    );
  }
}
