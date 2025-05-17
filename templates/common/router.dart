import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

// Feature routers
import 'features/auth/router.dart' as auth_router;
import 'features/home/router.dart' as home_router;
import 'features/notification/router.dart' as notification_router;

// Main application router
final appRouter = GoRouter(
  initialLocation: '/',
  debugLogDiagnostics: true,
  routes: [
    // Authentication routes
    ...auth_router.routes,
    
    // Home routes
    ...home_router.routes,
    
    // Notification routes
    ...notification_router.routes,
  ],
  errorBuilder: (context, state) => const NotFoundScreen(),
  // Define global redirect logic here
  redirect: (context, state) {
    // Auth logic can be added here
    return null;
  },
);

// 404 Not Found Screen
class NotFoundScreen extends StatelessWidget {
  const NotFoundScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Page Not Found'),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Text(
              '404',
              style: TextStyle(fontSize: 72, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 16),
            const Text(
              'Page not found',
              style: TextStyle(fontSize: 24),
            ),
            const SizedBox(height: 32),
            ElevatedButton(
              onPressed: () => context.go('/'),
              child: const Text('Go Home'),
            ),
          ],
        ),
      ),
    );
  }
}
