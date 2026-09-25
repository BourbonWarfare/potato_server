#include "script_component.hpp"

private _session = call FUNC(backendSession);
if (_session isEqualTo []) exitWith {};

params ["_message", "_sender"];
private _toSendFormatted = format ["%1: %2", _sender, _message];


[_session, "admin-message", _toSendFormatted] call EFUNC(interface,event_send);
