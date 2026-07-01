#include "script_component.hpp"

params ["_session"];

(EXTENSION callExtension ["backend:session:current", [_session]]) params ["_result", "_returnCode", "_errorCode"];
if (_errorCode != 0) exitWith {
    switch (_errorCode) do {
        case 102: {
            ERROR("Extension called with wrong parameter type");
        };
        case 301: {
            ERROR("Remote call has taken too long and has not been completed");
        };
    };
    [] // return
};

if (_returnCode == 9) exitWith {
    ERROR_1("Failed to get current session: %1",_result);
    [] // return
};

parseSimpleArray _result
