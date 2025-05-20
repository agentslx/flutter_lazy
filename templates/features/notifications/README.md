# Notification System Documentation

This documentation explains how the notification system works in the Flutter Lazy CLI templates.

## Overview

The notification system handles push notifications from Firebase Cloud Messaging (FCM) for both Android and iOS platforms. It manages:

1. Requesting notification permissions
2. Registering device tokens with the backend
3. Handling foreground, background, and terminated notifications
4. Managing notification UI elements
5. Clearing tokens on logout

## Architecture

### Services

- **NotificationService**: Core service for handling Firebase messaging and local notifications
  - Initializes Firebase and local notification plugins
  - Requests permissions from the user
  - Registers token with backend
  - Handles foreground, background, and terminated notifications

- **NotificationAuthService**: Connects authentication events with notification functions
  - Requests permissions after successful login
  - Clears notification tokens on logout

### Data Layer

- **NotificationRepository**: Interface for notification data operations
  - Sends push tokens to backend
  - Clears push tokens on logout
  - Gets notification list
  - Marks notifications as read

- **NotificationRemoteDataSource**: Implementation for API calls
  - Handles API requests for notification operations

### UI Components

- **NotificationBadge**: Shows unread notification counts
- **NotificationPage**: Lists all notifications
- **NotificationDetailsPage**: Shows notification details

## Authentication Integration

The notification system is integrated with authentication flow:

1. After successful login in `LoginCubit`, notification permissions are requested
2. On logout via `UserService`, notification tokens are cleared from the backend

## Usage

### Login Flow

The `LoginCubit` handles requesting notification permissions after successful authentication:

```dart
// In login_cubit.dart
Future<void> logInWithCredentials() async {
  // Authentication logic
  // ...
  
  if (successful) {
    // Request notification permissions
    await _notificationAuthService.onUserLoggedIn();
  }
}
```

### Logout Flow

The `UserService` handles clearing notification tokens during logout:

```dart
// In user_service.dart
Future<void> logout() async {
  // Clear notification token
  await _notificationAuthService.onUserLoggedOut();
  
  // Logout from backend
  await _authRepository.logout();
  
  // Clear local data
  // ...
}
```

### Adding Notification UI

Add the notification badge to your app bar:

```dart
AppBar(
  actions: [
    NotificationBadge(),
    // Other actions
  ],
)
```

### Handling Notification Taps

Notification taps are handled in the `NotificationService` class. You can customize the `_onTapLocalNotification` method to navigate to specific screens based on notification data.

## Important Notes

1. Ensure you have Firebase configured correctly in your project
2. iOS requires additional setup in Xcode for push notifications
3. Android requires proper setup in the AndroidManifest.xml
4. Background notification handler must be a top-level function
