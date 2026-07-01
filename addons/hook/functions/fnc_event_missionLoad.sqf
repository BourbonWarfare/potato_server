#include "script_component.hpp"

TRACE_1(time);

{
    GVAR(recruitsSeen) set [name _x, true];
} forEach call FUNC(recruits)
TRACE_1("Recruits at mission start",count GVAR(recruitsSeen));
