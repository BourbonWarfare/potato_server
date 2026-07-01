#include "script_component.hpp"

if !(isServer) exitWith {};
private _isBwmf = call FUNC(isBwmf);
if !(_isBwmf) exitWith {
    TRACE_1("not a BWMF mission, ignoring",_isBwmf);
};

GVAR(safeStartDisabled) = false;
GVAR(recruitsSeen) = createHashMap;
GVAR(missionName) = missionNameSource;
GVAR(worldName) = worldName;

[{time >= MISSION_LOAD_BUFFER_TIME}, LINKFUNC(event_missionLoad)] call CBA_fnc_waitUntilAndExecute;
[QPOTVAR(safeStartOff), LINKFUNC(event_safeStartOff)] call CBA_fnc_addEventHandler;

if (isDedicated) then {
    addMissionEventHandler ["MPEnded", LINKFUNC(event_missionEnded)];
} else {
    addMissionEventHandler ["Ended", LINKFUNC(event_missionEnded)];
};
