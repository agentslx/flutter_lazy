import 'dart:async';
import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:flutter/foundation.dart';

// Interface
abstract class PushNotificationModule {
  Future<void> initialize();
  Future<String?> getToken();
  Future<void> subscribeToTopic(String topic);
  Future<void> unsubscribeFromTopic(String topic);
  Stream<RemoteMessage> get onMessageStream;
  Stream<String> get onTokenRefreshStream;
  void dispose();
}

// Implementation using Firebase Messaging
class PushNotificationModuleImpl implements PushNotificationModule {
  final FirebaseMessaging _firebaseMessaging = FirebaseMessaging.instance;
  final StreamController<RemoteMessage> _messageStreamController = StreamController<RemoteMessage>.broadcast();
  final StreamController<String> _tokenStreamController = StreamController<String>.broadcast();
  
  @override
  Future<void> initialize() async {
    // Request permission
    NotificationSettings settings = await _firebaseMessaging.requestPermission(
      alert: true,
      badge: true,
      sound: true,
      provisional: false,
    );
    
    if (kDebugMode) {
      print('User granted permission: ${settings.authorizationStatus}');
    }
    
    // Listen for token refresh
    FirebaseMessaging.instance.onTokenRefresh.listen((token) {
      _tokenStreamController.add(token);
    });
    
    // Handle foreground messages
    FirebaseMessaging.onMessage.listen((RemoteMessage message) {
      _messageStreamController.add(message);
    });
    
    // Handle background messages
    FirebaseMessaging.onBackgroundMessage(_firebaseMessagingBackgroundHandler);
    
    // Get initial message (when app was terminated)
    RemoteMessage? initialMessage = await FirebaseMessaging.instance.getInitialMessage();
    if (initialMessage != null) {
      _messageStreamController.add(initialMessage);
    }
  }
  
  @override
  Future<String?> getToken() {
    return _firebaseMessaging.getToken();
  }
  
  @override
  Future<void> subscribeToTopic(String topic) {
    return _firebaseMessaging.subscribeToTopic(topic);
  }
  
  @override
  Future<void> unsubscribeFromTopic(String topic) {
    return _firebaseMessaging.unsubscribeFromTopic(topic);
  }
  
  @override
  Stream<RemoteMessage> get onMessageStream => _messageStreamController.stream;
  
  @override
  Stream<String> get onTokenRefreshStream => _tokenStreamController.stream;
  
  @override
  void dispose() {
    _messageStreamController.close();
    _tokenStreamController.close();
  }
}

// Background message handler
@pragma('vm:entry-point')
Future<void> _firebaseMessagingBackgroundHandler(RemoteMessage message) async {
  // This function will be called when the app is in the background or terminated
  print("Handling a background message: ${message.messageId}");
}
