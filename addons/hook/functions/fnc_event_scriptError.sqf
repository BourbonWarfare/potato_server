#include "script_component.hpp"
#define SEPERATOR +++++
#define FORMAT_STRING QUOTE(%1SEPERATOR%2SEPERATOR%3SEPERATOR%4SEPERATOR%5SEPERATOR)

private _session = call FUNC(backendSession);
if (_session isEqualTo []) exitWith {};

params ["_errorText", "_sourceFile", "_lineNumber", "_errorPos", "_content"];
private _toSendFormatted = format [FORMAT_STRING, _errorText, _sourceFile, _lineNumber, _errorPos, _content];

[_session, "script-error", _toSendFormatted] call EFUNC(interface,event_send);
