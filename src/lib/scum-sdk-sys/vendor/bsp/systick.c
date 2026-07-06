#include <stddef.h>
#include <stdint.h>

#include "scum.h"
#include "systick.h"

#define SYSTICK_BACKOFF 20U

typedef struct {
    systick_cb_t cb;
    uint64_t ticks;
} systick_vars_t;

static systick_vars_t _systick_vars = { 0 };

void systick_init(uint32_t ticks, systick_cb_t cb) {
    _systick_vars.cb = cb;

    SysTick->VAL = 0UL;
    SysTick->LOAD = ticks + SYSTICK_BACKOFF;

    /* Enable SysTick Exception and SysTick Timer */
    SysTick->CTRL = (SysTick_CTRL_CLKSOURCE_Msk |
                     SysTick_CTRL_TICKINT_Msk |
                     SysTick_CTRL_ENABLE_Msk);
}

uint32_t systick_count(void) {
    return _systick_vars.ticks;
}

void SysTick_Handler(void) {

    _systick_vars.ticks += SysTick->VAL;

    // Call callback if set
    if (_systick_vars.cb) {
        _systick_vars.cb();
    }
}
