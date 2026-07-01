#include "script_component.hpp"

params ["_token"];

(EXTENSION callExtension ["backend:auth:login", [_token]]) params ["_result", "_returnCode", "_errorCode"];
if (_errorCode != 0) exitWith {
    switch (_errorCode) do {
        case 102: {
            ERROR("Extension called with wrong parameter type");
        };
        case 301: {
            ERROR("Remote call has taken too long and has not been completed");
        };
    };
    [["expire_time",[1970,1,1,0,0,0,0]],["session_token","<null>"]] // return
};

parseSimpleArray _result
