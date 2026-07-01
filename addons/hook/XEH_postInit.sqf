#include "script_component.hpp"

if !(isServer) exitWith {};

GVAR(safeStartDisabled) = false;
GVAR(recruitsSeen) = createHashMap;
GVAR(missionName) = missionNameSource;
GVAR(worldName) = worldName;

[{time >= MISSION_LOAD_BUFFER_TIME}, LINKFUNC(event_missionLoad)] call CBA_fnc_waitUntilAndExecute;
[QPOTVAR(safeStartOff), LINKFUNC(event_safeStartOff)] call CBA_fnc_addEventHandler;
addMissionEventHandler ["MPEnded", LINKFUNC(event_missionEnded)];
