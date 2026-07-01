#include "script_component.hpp"

params ["_session", "_armaSession", "_missionName", "_worldName", "_orbat"];

(EXTENSION callExtension ["backend:session:finish_mission", [_session, _armaSession, _missionName, _worldName, _orbat]]) params ["_result", "_returnCode", "_errorCode"];
if (_errorCode != 0) exitWith {
    switch (_errorCode) do {
        case 102: {
            ERROR("Extension called with wrong parameter type");
        };
        case 301: {
            ERROR("Remote call has taken too long and has not been completed");
        };
    };
    false
};

if (_returnCode != 0) exitWith {
    ERROR_2("Failed to end mission (return %1), message: '%2'",_returnCode,_result);
    false
};

true
