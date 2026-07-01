#include "\z\potato\addons\core\script_macros.hpp"

#define POT_PREFIX potato

#define POTVAR(var) DOUBLES(POT_PREFIX,var)
#define QPOTVAR(var) QUOTE(DOUBLES(POT_PREFIX,var))

#define POTGVAR(module,var) TRIPLES(POT_PREFIX,module,var)
#define QPOTGVAR(module,var) QUOTE(POTGVAR(module,var))

#define POTFUNC(var1,var2) TRIPLES(DOUBLES(POT_PREFIX,var1),fnc,var2)
#define DPOTFUNC(var1,var2) TRIPLES(DOUBLES(POT_PREFIX,var1),fnc,var2)
#define QPOTFUNC(var1,var2) QUOTE(DPOTFUNC(var1,var2))
