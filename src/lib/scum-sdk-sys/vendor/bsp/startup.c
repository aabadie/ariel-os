#include <stdio.h>
#include <stdint.h>

#include "scum.h"
#include "scm3c_hw_interface.h"
#if defined(MODULE_OPTICAL)
#include "optical.h"
#endif

// linker symbols for memory sections
extern uint32_t _stext;
extern uint32_t _etext;
extern uint32_t _sbss;
extern uint32_t _ebss;
extern uint32_t _sdata;
extern uint32_t _edata;
extern uint32_t _sstack;
extern uint32_t _estack;

// the main function
int main(void);

// Newlib C library initialization
void __libc_init_array(void);

// exception handlers
void Reset_Handler(void);
void HardFault_Handler(void);
void NMI_Handler(void);

// Cortex-M0 core handlers
// clang-format off
void NMI_Handler(void)                              __attribute__((weak, alias("Dummy_Handler")));
void SVC_Handler(void)                              __attribute__((weak, alias("Dummy_Handler")));
void PendSV_Handler(void)                           __attribute__((weak, alias("Dummy_Handler")));
void SysTick_Handler(void)                          __attribute__((weak, alias("Dummy_Handler")));

/* External interrupts */
void UART_Handler(void)                             __attribute__((weak, alias("Dummy_Handler")));
void EXT_GPIO3_ACTIVEHIGH_DEBOUNCED_Handler(void)   __attribute__((weak, alias("Dummy_Handler")));
void EXT_OPTICAL_IRQ_IN_Handler(void)               __attribute__((weak, alias("Dummy_Handler")));
void ADC_Handler(void)                              __attribute__((weak, alias("Dummy_Handler")));
void RF_Handler(void)                               __attribute__((weak, alias("Dummy_Handler")));
void RFTIMER_Handler(void)                          __attribute__((weak, alias("Dummy_Handler")));
void RAWCHIPS_STARTVAL_Handler(void)                __attribute__((weak, alias("Dummy_Handler")));
void RAWCHIPS_32_Handler(void)                      __attribute__((weak, alias("Dummy_Handler")));
void OPTICAL_SFD_Handler(void)                      __attribute__((weak, alias("Dummy_Handler")));
void EXT_GPIO8_ACTIVEHIGH_Handler(void)             __attribute__((weak, alias("Dummy_Handler")));
void EXT_GPIO9_ACTIVELOW_Handler(void)              __attribute__((weak, alias("Dummy_Handler")));
void EXT_GPIO10_ACTIVELOW_Handler(void)             __attribute__((weak, alias("Dummy_Handler")));
// clang-format on

// vector table
typedef void (*vector_table_t)(void);
typedef struct {
    void *_estack;
    vector_table_t table[32];
} vectors_t;

extern const vectors_t _vectors;
const vectors_t _vectors __attribute__((used, section(".vectors"))) = {
    &_estack,
    {
        /* Exceptions */
        Reset_Handler,
        NMI_Handler,
        HardFault_Handler,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        SVC_Handler,
        0,
        0,
        PendSV_Handler,
        SysTick_Handler,

        /* SCUM external interrupts */
        UART_Handler,
        EXT_GPIO3_ACTIVEHIGH_DEBOUNCED_Handler,
        EXT_OPTICAL_IRQ_IN_Handler,
        ADC_Handler,
        0,
        0,
        RF_Handler,
        RFTIMER_Handler,
        RAWCHIPS_STARTVAL_Handler,
        RAWCHIPS_32_Handler,
        0,
        OPTICAL_SFD_Handler,
        EXT_GPIO8_ACTIVEHIGH_Handler,
        EXT_GPIO9_ACTIVELOW_Handler,
        EXT_GPIO10_ACTIVELOW_Handler,
        0,
    }
};

void HardFault_Handler(void) {
#ifndef NDEBUG
    puts("Hard Fault!");
#endif
    while (1) {}
}

void Dummy_Handler(void) {
#ifndef NDEBUG
    puts("Dummy handler!");
#endif
    while (1) {}
}

static void scum_init(void) {
    initialize_mote();
#ifndef NDEBUG
    crc_check();
#endif
#if defined(MODULE_OPTICAL)
    perform_calibration();
#endif
}

void Reset_Handler(void) {
    /* Initialize the data segment */
    uint32_t *pSrc = &_etext;
    uint32_t *pDest = &_sdata;
    if (pSrc != pDest) {
        for (; pDest < &_edata;) {
            *pDest++ = *pSrc++;
        }
    }

    /* Clear the zero segment */
    for (pDest = &_sbss; pDest < &_ebss;) {
        *pDest++ = 0;
    }

    // Initialize constructors (but calling it triggers a Hard Fault)
    // __libc_init_array();

#ifndef NDEBUG
    puts("");
    puts("-------------------");
    puts("-- Booting SCUM! --");
    puts("-------------------");
    puts("");
#endif

    // Initialize the system
    scum_init();

    main();

    while (1) {}
}
