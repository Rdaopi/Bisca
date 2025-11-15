import 'package:flutter/material.dart';

import '../features/auth/views/login_screen.dart';
import '../features/auth/views/register_screen.dart';
import '../features/game/views/game_screen.dart';
import '../features/lobby/views/lobby_screen.dart';
import '../features/scoreboard/views/scoreboard_screen.dart';

class AppRouter {
  static const login = '/login';
  static const register = '/register';
  static const lobby = '/lobby';
  static const game = '/game';
  static const scoreboard = '/scoreboard';

  static Route<dynamic> onGenerateRoute(RouteSettings settings) {
    switch (settings.name) {
      case login:
        return MaterialPageRoute<void>(builder: (_) => const LoginScreen());
      case register:
        return MaterialPageRoute<void>(builder: (_) => const RegisterScreen());
      case game:
        return MaterialPageRoute<void>(builder: (_) => const GameScreen());
      case scoreboard:
        return MaterialPageRoute<void>(
          builder: (_) => const ScoreboardScreen(),
        );
      case lobby:
      default:
        return MaterialPageRoute<void>(builder: (_) => const LobbyScreen());
    }
  }
}
