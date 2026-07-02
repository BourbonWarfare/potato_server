#include "script_component.hpp"

// Handle getting the current session, plus re-authorizing if it is expired
// Must handle case where session is not valid returned

private _botTokenVariable = "potato_botToken";
TRACE_1("backendSession",_botTokenVariable);

private _botToken = profileNamespace getVariable [_botTokenVariable, ""];
if (_botToken isEqualTo "") exitWith {
    WARNING_1("No bot-token has been defined: `profileNameSpace getVariable %1`. Unable to get session",_botTokenVariable);
    []
};

if (isNil QGVAR(session)) then {
    GVAR(session) = [_botToken] call EFUNC(interface,auth_login);
};

private _timeToExpire = [GVAR(session)] call EFUNC(interface,auth_expiresIn);
if (_timeToExpire <= SESSION_EXPIRE_THRESHOLD) then {
    GVAR(session) = [_botToken] call EFUNC(interface,auth_login);
};

GVAR(session) // return
