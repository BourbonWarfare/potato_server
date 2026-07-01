#include "script_component.hpp"

// Handle getting the current session, plus re-authorizing if it is expired
// Must handle case where session is not valid returned

private _botToken = profileNamespace getVariable [format["%1_botToken", ADDON], ""];
if (_botToken isEqualTo "") exitWith {
    ERROR("No bot-token has been defined in `profileNameSpace`. Unable to get session");
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
