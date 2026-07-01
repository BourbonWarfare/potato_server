#include "script_component.hpp"

TRACE_1("event_missionEnded",GVAR(safeStartDisabled));
if !(GVAR(safeStartDisabled)) exitWith {};

{
    GVAR(recruitsSeen) set [name _x, true];
} forEach call FUNC(recruits);
TRACE_1("Recruits at mission end",count GVAR(recruitsSeen));

private _session = call FUNC(backendSession);
if (_session isEqualTo []) exitWith {};

private _currentArmaSession = [_session] call EFUNC(interface,session_current);

[_session, _currentArmaSession, GVAR(missionName), GVAR(worldName), call FUNC(orbat)] call EFUNC(interface,session_endMission);
