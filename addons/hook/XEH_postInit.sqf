#include "script_component.hpp"

if !(isServer) exitWith {};
private _isBwmf = call FUNC(isBwmf); // why is this a function? you call it once, just store a GVAR if you think you'll need it
if !(_isBwmf) exitWith {
    TRACE_1("not a BWMF mission, ignoring",_isBwmf);
};

GVAR(safeStartDisabled) = false;
GVAR(recruitsSeen) = createHashMap;
GVAR(missionName) = missionNameSource;
GVAR(worldName) = worldName;

[{time >= MISSION_LOAD_BUFFER_TIME}, LINKFUNC(event_missionLoad)] call CBA_fnc_waitUntilAndExecute;// why wait for so long on this one?
[QPOTVAR(safeStartOff), LINKFUNC(event_safeStartOff)] call CBA_fnc_addEventHandler; // we use a fallback here in the old extension for a reason. A

if (isDedicated) then {
    addMissionEventHandler ["MPEnded", LINKFUNC(event_missionEnded)];
} else { // do we ever have a case where it isn't on the dedicated we want the bot to deal with?
    addMissionEventHandler ["Ended", LINKFUNC(event_missionEnded)];
};
