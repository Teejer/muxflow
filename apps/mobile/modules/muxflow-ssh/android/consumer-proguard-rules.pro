# R8 in the app's release build must not touch sshj or Bouncy Castle: both
# find their algorithms through provider registration and reflection, so
# shrinking removes exactly the Ed25519 classes the key exchange asks for by
# name, and every connect fails with "no such algorithm: Ed25519 for provider BC".
-keep class net.schmizz.sshj.** { *; }
-keep class com.hierynomus.** { *; }
-keep class org.bouncycastle.** { *; }
-keep class org.slf4j.** { *; }
-keep class uk.uuid.slf4j.** { *; }
-dontwarn org.bouncycastle.**
-dontwarn javax.naming.**
-dontwarn java.lang.management.**
-dontwarn javax.security.auth.**
-dontwarn org.ietf.jgss.**
