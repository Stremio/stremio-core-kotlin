#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <Foundation/Foundation.h>

void initialize_rust(void);

NSObject *initializeNative(NSString *device_info);

void dispatchNative(NSObject *action_protobuf);

NSObject *getStateNative(int32_t field);

NSObject *decodeStreamDataNative(NSString *field);

void sendNextAnalyticsBatch(void);

NSString *getVersionNative(void);

void releaseObjectNative(NSObject *object);
