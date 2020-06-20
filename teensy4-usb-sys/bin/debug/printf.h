#ifndef PRINTF_SHIM_H_
#define PRINTF_SHIM_H_

// NOTE(mciantyre) There are some printfs throughout the implementation.
// This header was implemented to suppress those calls.

#define printf(...)

#endif // PRINTF_SHIM_H_