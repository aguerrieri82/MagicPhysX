rustup target add aarch64-linux-android

REM install cargo-ndk
 
SET ANDROID_NDK_ROOT=C:\Android\ndk\26.1.10909125\
SET NDK_PROJECT_PATH=%CD%

cargo ndk --platform 26 --target aarch64-linux-android build --release 

REM call %ANDROID_NDK_ROOT%\build\ndk-build
