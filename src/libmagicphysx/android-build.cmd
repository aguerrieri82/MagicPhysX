rustup target add aarch64-linux-android
install cargo-ndk
 
SET ANDROID_NDK_ROOT=C:\Android\ndk\26.1.10909125\

cargo ndk --platform 26 --target aarch64-linux-android build --release 