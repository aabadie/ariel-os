#ifndef __TUNING_H
#define __TUNING_H

#include <stdint.h>

/// Callback function prototype, it is called when the systick interrupt fires
typedef void (*systick_cb_t)(void);

void systick_init(uint32_t ticks, systick_cb_t cb);

uint32_t systick_count(void);

#endif
