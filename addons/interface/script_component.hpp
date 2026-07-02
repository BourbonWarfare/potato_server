#define COMPONENT interface
#define COMPONENT_BEAUTIFIED Interface
#include "\z\potato_server\addons\main\script_mod.hpp"

// #define DEBUG_MODE_FULL
// #define DISABLE_COMPILE_CACHE
// #define CBA_DEBUG_SYNCHRONOUS
// #define ENABLE_PERFORMANCE_COUNTERS

#ifdef DEBUG_ENABLED_INTERFACE
    #define DEBUG_MODE_FULL
#endif

#ifdef DEBUG_SETTINGS_INTERFACE
    #define DEBUG_SETTINGS DEBUG_SETTINGS_INTERFACE
#endif

#include "script_macros.hpp"

#include "\z\potato_server\addons\main\script_macros.hpp"
