
LOCAL_PATH := $(call my-dir)

LOCAL_MODULE := libmagicphysx-static
LOCAL_SRC_FILES := $(LOCAL_PATH)/../target/aarch64-linux-android/release/libmagicphysx.a
include $(PREBUILT_STATIC_LIBRARY)

include $(CLEAR_VARS)

LOCAL_MODULE := magicphysx

LOCAL_WHOLE_STATIC_LIBRARIES := libmagicphysx-static

include $(BUILD_SHARED_LIBRARY)
