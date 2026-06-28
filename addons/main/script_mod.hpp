// COMPONENT should be defined in the script_component.hpp and included BEFORE this hpp

#define MAINPREFIX z
#define PREFIX potato_server

#include "script_version.hpp"

#define VERSION MAJOR.MINOR
#define VERSION_STR MAJOR.MINOR.PATCHLVL.BUILD
#define VERSION_AR MAJOR,MINOR,PATCHLVL,BUILD

#define POTATO_SERVER_TAG POTATO_SERVER

// MINIMAL required version for the Mod. Components can specify others..
#define REQUIRED_VERSION 2.20

#ifdef COMPONENT_BEAUTIFIED
    #define COMPONENT_NAME QUOTE(potato_server - COMPONENT_BEAUTIFIED)
#else
    #define COMPONENT_NAME QUOTE(potato_server - COMPONENT)
#endif
