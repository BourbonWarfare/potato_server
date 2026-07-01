#include "script_component.hpp"

// Return a list of recruits

allUnits select {(alive _x) && {isPlayer _x} && {[_x] call POTFUNC(recruits,isNotMember)}};