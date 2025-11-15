import 'package:flutter/material.dart';

class ScoreboardScreen extends StatelessWidget {
  const ScoreboardScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Punteggi')),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: const <Widget>[
          ListTile(
            title: Text('Giocatore 1'),
            subtitle: Text('Previsione: 2 • Prese: 1'),
          ),
          ListTile(
            title: Text('Giocatore 2'),
            subtitle: Text('Previsione: 1 • Prese: 1'),
          ),
        ],
      ),
    );
  }
}
