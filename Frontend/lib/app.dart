import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'routes/app_router.dart';

class BiscaApp extends ConsumerWidget {
  const BiscaApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return MaterialApp(
      title: 'Bisca',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      onGenerateRoute: AppRouter.onGenerateRoute,
      initialRoute: AppRouter.lobby,
    );
  }
}
