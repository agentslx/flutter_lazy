import 'package:go_router/go_router.dart';

import 'ui/pages/test_feat_detail_page.dart';
import 'ui/pages/test_feat_page.dart';

class TestFeatRouter {
  // Define routes as constants for easy reference
  static const String base = '/test_feat';
  static const String detail = 'detail';
  
  // For building full paths
  static String getDetailPath(String id) => '$base/$detail?id=$id';
  
  // Define all routes for this feature
  static final List<RouteBase> routes = [
    GoRoute(
      path: base,
      builder: (context, state) => const TestFeatPage(),
      routes: [
        GoRoute(
          path: detail,
          builder: (context, state) {
            final id = state.uri.queryParameters['id'] ?? '';
            return TestFeatDetailPage(id: id);
          },
        ),
      ],
    ),
  ];
}
