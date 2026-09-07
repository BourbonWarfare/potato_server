#include "script_component.hpp"

// Return a list of recruits

([] call CBA_fnc_players) select {[_x] call POTFUNC(recruits,isNotMember)};
