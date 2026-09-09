#include "script_component.hpp"

params [["_id", "", ["", objNull]]];

if (!(_id isEqualType objNull)) then {
    _id = getPlayerUID _id;
};
if (_id == "") exitWith {false};

(EXTENSION callExtension ["backend:member:is_member", [_id]]) params ["_result", "_returnCode", "_errorCode"];

_result == "true"
