# cargo-ndk build rules for the stardew-view crate.

NDK_TARGETS := arm64-v8a armeabi-v7a x86 x86_64
AARCH64 := aarch64-linux-android
ARMV7 := armv7-linux-androideabi
X86_64 := x86_64-linux-android
X86 := i686-linux-android

.PHONY: all armeabi-v8a armeabi-v7a x86 x86_64 clean

all: arm64-v8a armeabi-v7a x86 x86_64

arm64-v8a:
	cargo ndk -t aarch64-linux-android build --release
	mkdir -p libs/arm64-v8a
	cp ../target/aarch64-linux-android/release/libstardew_view.so libs/arm64-v8a/

armeabi-v7a:
	cargo ndk -t armv7-linux-androideabi build --release
	mkdir -p libs/armeabi-v7a
	cp ../target/armv7-linux-androideabi/release/libstardew_view.so libs/armeabi-v7a/

x86:
	cargo ndk -t i686-linux-android build --release
	mkdir -p libs/x86
	cp ../target/i686-linux-android/release/libstardew_view.so libs/x86/

x86_64:
	cargo ndk -t x86_64-linux-android build --release
	mkdir -p libs/x86_64
	cp ../target/x86_64-linux-android/release/libstardew_view.so libs/x86_64/

clean:
	rm -rf libs/arm64-v8a/*.so libs/armeabi-v7a/*.so libs/x86/*.so libs/x86_64/*.so