import 'package:bisca_frontend/app.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('renders lobby screen', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: BiscaApp()));
    expect(find.text('Lobby'), findsOneWidget);
  });
}
