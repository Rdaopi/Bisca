import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../routes/app_router.dart';
import '../controllers/auth_controller.dart';

class RegisterScreen extends ConsumerStatefulWidget {
  const RegisterScreen({super.key});

  @override
  ConsumerState<RegisterScreen> createState() => _RegisterScreenState();
}

class _RegisterScreenState extends ConsumerState<RegisterScreen> {
  final TextEditingController _emailController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();
  final TextEditingController _usernameController = TextEditingController();

  @override
  void dispose() {
    _emailController.dispose();
    _passwordController.dispose();
    _usernameController.dispose();
    super.dispose();
  }

  Future<void> _onRegister() async {
    await ref
        .read(authControllerProvider.notifier)
        .register(
          _emailController.text,
          _passwordController.text,
          _usernameController.text,
        );
    if (!mounted) {
      return;
    }
    Navigator.of(context).pushReplacementNamed(AppRouter.login);
  }

  @override
  Widget build(BuildContext context) {
    final AuthState state = ref.watch(authControllerProvider);
    return Scaffold(
      appBar: AppBar(title: const Text('Registrati')),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: <Widget>[
            TextField(
              controller: _usernameController,
              decoration: const InputDecoration(labelText: 'Username'),
            ),
            const SizedBox(height: 12),
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
                  : () => _onRegister(),
              child: const Text('Crea account'),
            ),
            TextButton(
              onPressed: () {
                Navigator.of(context).pushReplacementNamed(AppRouter.login);
              },
              child: const Text('Hai gia un account? Accedi'),
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
