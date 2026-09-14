#include "script_component.hpp"
// Generate a formatted ORBAT from all players

private _groups = [];

{
    private _members = [];
    {
        _members pushBack [
            ["variable", str _x],
            ["name", name _x],
            ["is_member", !(_x call POTFUNC(recruits,isNotMember))],
            ["rank", rankId _x],
            ["steam_id", getPlayerUID _x]
        ];
    } forEach (units _x select { isPlayer _x });

    _groups pushBack [
        ["name", groupId _x],
        ["side", str side _x],
        ["leader", getPlayerUID leader _x],
        ["members", _members]
    ];
} forEach (allGroups select { isPlayer leader _x });

private _spectators = allPlayers select {_x isKindOf QPOTGVAR(spectate,spectator)} apply {
    [
        ["variable", str _x],
        ["name", name _x],
        ["is_member", !(_x call POTFUNC(recruits,isNotMember))],
        ["rank", rankId _x],
        ["steam_id", getPlayerUID _x]
    ];
};
if (_spectators isNotEqualTo []) then {
    _groups pushBack [
        ["name", "Spectators"],
        ["side", "LOGIC"],
        ["leader", "0"],
        ["members", _spectators]
    ];
};

[["groups", _groups]]
