import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../routes/app_router.dart';
import '../controllers/auth_controller.dart';

class LoginScreen extends ConsumerStatefulWidget {
  const LoginScreen({super.key});

  @override
  ConsumerState<LoginScreen> createState() => _LoginScreenState();
}

class _LoginScreenState extends ConsumerState<LoginScreen> {
  final TextEditingController _emailController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();

  @override
  void dispose() {
    _emailController.dispose();
    _passwordController.dispose();
    super.dispose();
  }

  Future<void> _onLogin() async {
    await ref
        .read(authControllerProvider.notifier)
        .login(_emailController.text, _passwordController.text);
    if (!mounted) {
      return;
    }
    Navigator.of(context).pushReplacementNamed(AppRouter.lobby);
  }

  @override
  Widget build(BuildContext context) {
    final AuthState state = ref.watch(authControllerProvider);
    return Scaffold(
      appBar: AppBar(title: const Text('Accedi')),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: <Widget>[
            TextField(
              controller: _emailController,
              decoration: const InputDecoration(labelText: 'Email'),
            ),
            const SizedBox(height: 12),
            TextField(
              controller: _passwordController,
              decoration: const InputDecoration(labelText: 'Password'),
              obscureText: true,
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: state.status == AuthStatus.loading
                  ? null
                  : () => _onLogin(),
              child: const Text('Entra'),
            ),
            TextButton(
              onPressed: () {
                Navigator.of(context).pushNamed(AppRouter.register);
              },
              child: const Text('Registrati'),
            ),
            if (state.message != null) ...<Widget>[
              const SizedBox(height: 16),
              Text(state.message!, style: const TextStyle(color: Colors.red)),
            ],
          ],
        ),
      ),
    );
  }
}
