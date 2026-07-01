#include "script_component.hpp"

params ["_session"];

(EXTENSION callExtension ["backend:auth:session_expire_time", [_session]]) params ["_result", "_returnCode", "_errorCode"];
if (_errorCode != 0) exitWith {
    switch (_errorCode) do {
        case 102: {
            ERROR("Extension called with wrong parameter type");
        };
        case 301: {
            ERROR("Remote call has taken too long and has not been completed");
        };
    };
    0.0 // return
};

parseNumber _result
