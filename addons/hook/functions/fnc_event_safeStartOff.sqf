#include "script_component.hpp"

TRACE_2("event_safeStartOff",QPOTVAR(safeStartOff),GVAR(safeStartDisabled));
if (GVAR(safeStartDisabled)) exitWith {};

{
    GVAR(recruitsSeen) set [name _x, true];
} forEach call FUNC(recruits);
TRACE_1("Recruits at SS off",count GVAR(recruitsSeen));

private _session = call FUNC(backendSession);
if (_session isEqualTo []) exitWith {};
GVAR(safeStartDisabled) = true;

private _currentArmaSession = [_session] call EFUNC(interface,session_current);

GVAR(safeStartOrbat) = call FUNC(orbat);
[_session, _currentArmaSession, GVAR(missionName), GVAR(worldName), GVAR(safeStartOrbat)] call EFUNC(interface,session_safeStartDisabled);
