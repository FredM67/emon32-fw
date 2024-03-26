#pragma once

/*! @brief Set up the SAMD clock system
 *         The following architecture is used:
 *         OSC32 -> ClkGen 1 -> Mux 0 -> DFLL48 -> ClkGen 0 (core clock)
 *         OSC8M -> ClkGen 3 -> (peripheral clock)
 */
void clkSetup(void);
