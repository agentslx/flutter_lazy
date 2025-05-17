enum Flavor {
  dev,
  stage,
  prod,
}

class F {
  static Flavor? appFlavor;

  static String get title {
    switch (appFlavor) {
      case Flavor.dev:
        return 'Dev App';
      case Flavor.stage:
        return 'Stage App';
      case Flavor.prod:
        return 'App';
      default:
        return 'App';
    }
  }

  static String get baseUrl {
    switch (appFlavor) {
      case Flavor.dev:
        return 'https://dev-api.example.com';
      case Flavor.stage:
        return 'https://stage-api.example.com';
      case Flavor.prod:
        return 'https://api.example.com';
      default:
        return 'https://api.example.com';
    }
  }
  
  static bool get isDev => appFlavor == Flavor.dev;
  static bool get isStage => appFlavor == Flavor.stage;
  static bool get isProd => appFlavor == Flavor.prod;
}
