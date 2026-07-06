#ifndef SCUM_H
#define SCUM_H

#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif

#define __CM0_REV              0x0000U
#define __NVIC_PRIO_BITS       2U
#define __Vendor_SysTickConfig 0U

/* ========================================================================= */
/* ============           Interrupt Number Definition           ============ */
/* ========================================================================= */

typedef enum IRQn {
    /* ================     Cortex-M Core Exception Numbers     ================ */
    Reset_IRQn = -15,          /*  1 Reset Vector
                                   invoked on Power up and warm reset */
    NonMaskableInt_IRQn = -14, /*  2 Non maskable Interrupt
                                     cannot be stopped or preempted */
    HardFault_IRQn = -13,      /*  3 Hard Fault
                                     all classes of Fault */
    SVCall_IRQn = -5,          /* 11 System Service Call via SVC instruction */
    DebugMonitor_IRQn = -4,    /* 12 Debug Monitor */
    PendSV_IRQn = -2,          /* 14 Pendable request for system service */
    SysTick_IRQn = -1,         /* 15 System Tick Timer */

    /* ================        SCUM Interrupt Numbers       ================ */

    UART_IRQn = 0,                           /* 0 UART interrupt */
    EXT_GPIO3_ACTIVEHIGH_DEBOUNCED_IRQn = 1, /* 1 GPIO3 interrupt */
    EXT_OPTICAL_IRQ_IN_IRQn = 2,             /* 2 Optical interrupt */
    ADC_IRQn = 3,                            /* 3 ADC interrupt */
    RF_IRQn = 6,                             /* 6 RF interrupt */
    RFTIMER_IRQn = 7,                        /* 7 RF Timer interrupt */
    RAWCHIPS_STARTVAL_IRQn = 8,              /* 8 RAWCHIPS start value interrupt */
    RAWCHIPS_32_IRQn = 9,                    /* 9 RAWCHIPS 32-bit interrupt */
    OPTICAL_SFD_IRQn = 11,                   /* 11 Optical SFD interrupt */
    EXT_GPIO8_ACTIVEHIGH_IRQn = 12,          /* 12 GPIO8 interrupt */
    EXT_GPIO9_ACTIVELOW_IRQn = 13,           /* 13 GPIO9 interrupt */
    EXT_GPIO10_ACTIVELOW_IRQn = 14,          /* 14 GPIO10 interrupt */

    TOTAL_IRQn = 15, /* 15 number of external interrupts */
} IRQn_Type;

/* ========================================================================= */
/* ============      Processor and Core Peripheral Section      ============ */
/* ========================================================================= */

/* ================ Start of section using anonymous unions ================ */
#if defined(__CC_ARM)
#pragma push
#pragma anon_unions
#elif defined(__ICCARM__)
#pragma language = extended
#elif defined(__ARMCC_VERSION) && (__ARMCC_VERSION >= 6010050)
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wc11-extensions"
#pragma clang diagnostic ignored "-Wreserved-id-macro"
#elif defined(__GNUC__)
/* anonymous unions are enabled by default */
#elif defined(__TMS470__)
/* anonymous unions are enabled by default */
#elif defined(__TASKING__)
#pragma warning 586
#elif defined(__CSMC__)
/* anonymous unions are enabled by default */
#else
#warning Not supported compiler type
#endif

#include <stdint.h>
#include <core_cm0.h>

/* ========================================================================= */
/* ============       Device Specific Peripheral Section        ============ */
/* ========================================================================= */

/* ========================================================================= */
/* ============                        RF                       ============ */
/* ========================================================================= */

typedef struct {
    __IOM uint32_t CONTROL;      /* Offset: 0x000 (R/W) Control Register */
    __IM uint32_t STATUS;        /* Offset: 0x004 (R/ ) Status Register */
    __IOM uint32_t TX_DATA_ADDR; /* Offset: 0x008 (R/W) Transmit Data Address Register */
    __IOM uint32_t TX_PACK_LEN;  /* Offset: 0x00C (R/W) Transmit Packet Length Register */
    __IM uint32_t INT;           /* Offset: 0x010 (R/ ) Interrupt Register */
    __IOM uint32_t INT_CONFIG;   /* Offset: 0x014 (R/W) Interrupt Configuration Register */
    __IOM uint32_t INT_CLEAR;    /* Offset: 0x018 (R/W) Interrupt Clear Register */
    __IM uint32_t ERROR;         /* Offset: 0x01C (R/ ) Error Register */
    __IOM uint32_t ERROR_CONFIG; /* Offset: 0x020 (R/W) Error Configuration Register */
    __IOM uint32_t ERROR_CLEAR;  /* Offset: 0x024 (R/W) Error Clear Register */
} SCUM_RF_TypeDef;

/* ========================================================================= */
/* ============                   RFTIMER                       ============ */
/* ========================================================================= */

typedef struct {
    __IOM uint32_t CONTROL;            /* Offset: 0x000 (R/W) Control Register */
    __IM uint32_t COUNTER;             /* Offset: 0x004 (R/ ) Counter Register */
    __IOM uint32_t MAX_COUNT;          /* Offset: 0x008 (R/W) Max Count Register */
    __IM uint32_t RESERVED;            /* Offset: 0x00C (R/ ) Reserved */
    __IOM uint32_t COMPARE[8];         /* Offset: 0x010 (R/W) Compare Register */
    __IOM uint32_t COMPARE_CONTROL[8]; /* Offset: 0x030 (R/W) Compare Control Register */
    __IOM uint32_t CAPTURE[4];         /* Offset: 0x050 (R/ ) Capture Register */
    __IOM uint32_t CAPTURE_CONTROL[4]; /* Offset: 0x060 (R/W) Capture Control Register */
    __IOM uint32_t INT;                /* Offset: 0x070 (R/ ) Interrupt Register */
    __IOM uint32_t INT_CLEAR;          /* Offset: 0x074 (R/W) Interrupt Clear Register */
} SCUM_RFTIMER_TypeDef;

/* ========================================================================= */
/* ============                      UART                       ============ */
/* ========================================================================= */

typedef struct {
    __IOM uint32_t DATA; /* Offset: 0x000 (R/W) Transmit/Receive Data Register */
} SCUM_UART_TypeDef;

/* ========================================================================= */
/* ============                Bit fields                       ============ */
/* ========================================================================= */

// /* <DeviceAbbreviation>_TMR LOAD Register Definitions */
// #define <DeviceAbbreviation>_TMR_LOAD_Pos              0
// #define <DeviceAbbreviation>_TMR_LOAD_Msk             (0xFFFFFFFFUL /*<< <DeviceAbbreviation>_TMR_LOAD_Pos*/)

// /* <DeviceAbbreviation>_TMR VALUE Register Definitions */
// #define <DeviceAbbreviation>_TMR_VALUE_Pos             0
// #define <DeviceAbbreviation>_TMR_VALUE_Msk            (0xFFFFFFFFUL /*<< <DeviceAbbreviation>_TMR_VALUE_Pos*/)

// /* <DeviceAbbreviation>_TMR CONTROL Register Definitions */
// #define <DeviceAbbreviation>_TMR_CONTROL_SIZE_Pos      1
// #define <DeviceAbbreviation>_TMR_CONTROL_SIZE_Msk     (1UL << <DeviceAbbreviation>_TMR_CONTROL_SIZE_Pos)

// #define <DeviceAbbreviation>_TMR_CONTROL_ONESHOT_Pos   0
// #define <DeviceAbbreviation>_TMR_CONTROL_ONESHOT_Msk  (1UL /*<< <DeviceAbbreviation>_TMR_CONTROL_ONESHOT_Pos*/)

// ==== RFCONTROLLER interruption bit configuration

#define TX_LOAD_DONE_INT_EN           0x0001
#define TX_SFD_DONE_INT_EN            0x0002
#define TX_SEND_DONE_INT_EN           0x0004
#define RX_SFD_DONE_INT_EN            0x0008
#define RX_DONE_INT_EN                0x0010
#define TX_LOAD_DONE_RFTIMER_PULSE_EN 0x0020
#define TX_SFD_DONE_RFTIMER_PULSE_EN  0x0040
#define TX_SEND_DONE_RFTIMER_PULSE_EN 0x0080
#define RX_SFD_DONE_RFTIMER_PULSE_EN  0x0100
#define RX_DONE_RFTIMER_PULSE_EN      0x0200

// ==== RFCONTROLLER error bit configuration

#define TX_OVERFLOW_ERROR_EN 0x001
#define TX_CUTOFF_ERROR_EN   0x002
#define RX_OVERFLOW_ERROR_EN 0x004
#define RX_CRC_ERROR_EN      0x008
#define RX_CUTOFF_ERROR_EN   0x010

// ==== RFCONTROLLER control operation

#define TX_LOAD  0x01
#define TX_SEND  0x02
#define RX_START 0x04
#define RX_STOP  0x08
#define RF_RESET 0x10

// ==== RFCONTROLLER interruption flag

#define TX_LOAD_DONE_INT 0x01
#define TX_SFD_DONE_INT  0x02
#define TX_SEND_DONE_INT 0x04
#define RX_SFD_DONE_INT  0x08
#define RX_DONE_INT      0x10

// ==== RFCONTROLLER error flag

#define TX_OVERFLOW_ERROR 0x01
#define TX_CUTOFF_ERROR   0x02
#define RX_OVERFLOW_ERROR 0x04
#define RX_CRC_ERROR      0x08
#define RX_CUTOFF_ERROR   0x10

// ==== RFTIMER compare control bit

#define RFTIMER_COMPARE_ENABLE           0x01
#define RFTIMER_COMPARE_INTERRUPT_ENABLE 0x02
#define RFTIMER_COMPARE_TX_LOAD_ENABLE   0x04
#define RFTIMER_COMPARE_TX_SEND_ENABLE   0x08
#define RFTIMER_COMPARE_RX_START_ENABLE  0x10
#define RFTIMER_COMPARE_RX_STOP_ENABLE   0x20

// ==== RFTIMER capture control bit

#define RFTIMER_CAPTURE_INTERRUPT_ENABLE       0x01
#define RFTIMER_CAPTURE_INPUT_SEL_SOFTWARE     0x02
#define RFTIMER_CAPTURE_INPUT_SEL_TX_LOAD_DONE 0x04
#define RFTIMER_CAPTURE_INPUT_SEL_TX_SFD_DONE  0x08
#define RFTIMER_CAPTURE_INPUT_SEL_TX_SEND_DONE 0x10
#define RFTIMER_CAPTURE_INPUT_SEL_RX_SFD_DONE  0x20
#define RFTIMER_CAPTURE_INPUT_SEL_RX_DONE      0x40
#define RFTIMER_CAPTURE_NOW                    0x80

// ==== RFTIMER control bit

#define RFTIMER_REG__CONTROL_ENABLE           0x01
#define RFTIMER_REG__CONTROL_INTERRUPT_ENABLE 0x02
#define RFTIMER_REG__CONTROL_COUNT_RESET      0x04

// ==== RFTIMER interruption flag

#define RFTIMER_REG__INT_COMPARE0_INT (1 << 0)
#define RFTIMER_REG__INT_COMPARE1_INT (1 << 1)
#define RFTIMER_REG__INT_COMPARE2_INT (1 << 2)
#define RFTIMER_REG__INT_COMPARE3_INT (1 << 3)
#define RFTIMER_REG__INT_COMPARE4_INT (1 << 4)
#define RFTIMER_REG__INT_COMPARE5_INT (1 << 5)
#define RFTIMER_REG__INT_COMPARE6_INT (1 << 6)
#define RFTIMER_REG__INT_COMPARE7_INT (1 << 7)

#define RFTIMER_REG__INT_CAPTURE0_INT 0x0100
#define RFTIMER_REG__INT_CAPTURE1_INT 0x0200
#define RFTIMER_REG__INT_CAPTURE2_INT 0x0400
#define RFTIMER_REG__INT_CAPTURE3_INT 0x0800

#define RFTIMER_REG__INT_CAPTURE0_OVERFLOW_INT 0x1000
#define RFTIMER_REG__INT_CAPTURE1_OVERFLOW_INT 0x2000
#define RFTIMER_REG__INT_CAPTURE2_OVERFLOW_INT 0x4000
#define RFTIMER_REG__INT_CAPTURE3_OVERFLOW_INT 0x8000

/* --------  End of section using anonymous unions and disabling warnings  -------- */
#if defined(__CC_ARM)
#pragma pop
#elif defined(__ICCARM__)
/* leave anonymous unions enabled */
#elif (defined(__ARMCC_VERSION) && (__ARMCC_VERSION >= 6010050))
#pragma clang diagnostic pop
#elif defined(__GNUC__)
/* anonymous unions are enabled by default */
#elif defined(__TMS470__)
/* anonymous unions are enabled by default */
#elif defined(__TASKING__)
#pragma warning restore
#elif defined(__CSMC__)
/* anonymous unions are enabled by default */
#else
#warning Not supported compiler type
#endif

/* ========================================================================= */
/* ============     Device Specific Peripheral Address Map      ============ */
/* ========================================================================= */

// ========================== DMA Registers ===================================

/* Peripheral and SRAM base address */
#define SCUM_CODE_BASE (0x00000000UL) /* (CODE  ) Base Address */
#define SCUM_SRAM_BASE (0x20000000UL) /* (SRAM  ) Base Address */
#define SCUM_AHB_BASE  (0x40000000UL) /* (AHB   ) Base Address */
#define SCUM_APB_BASE  (0x50000000UL) /* (APB   ) Base Address */

/* Peripheral memory map */
#define SCUM_RF_BASE      (SCUM_AHB_BASE)                /* (RF    ) Base Address */
#define SCUM_DMA_BASE     (SCUM_AHB_BASE + 0x01000000UL) /* (DMA   ) Base Address */
#define SCUM_RFTIMER_BASE (SCUM_AHB_BASE + 0x02000000UL) /* (RFTIMER) Base Address */

#define SCUM_RF_BASE (SCUM_AHB_BASE) /* (RF    ) Base Address */

#define SCUM_ADC_START_BASE  (SCUM_APB_BASE)                /* (ADC   ) START Register Base Address */
#define SCUM_ADC_DATA_BASE   (SCUM_APB_BASE + 0x040000)     /* (ADC   ) DATA Register Base Address */
#define SCUM_UART_BASE       (SCUM_APB_BASE + 0x01000000UL) /* (UART  ) Base Address */
#define SCUM_ANALOG_CFG_BASE (SCUM_APB_BASE + 0x02000000UL) /* (UART  ) Base Address */
#define SCUM_GPIO_BASE       (SCUM_APB_BASE + 0x03000000UL) /* (GPIO  ) Base Address */

/* ========================================================================= */
/* ============             Peripheral declaration              ============ */
/* ========================================================================= */

#define SCUM_RF      ((SCUM_RF_TypeDef *)SCUM_RF_BASE)
#define SCUM_RFTIMER ((SCUM_RFTIMER_TypeDef *)SCUM_RFTIMER_BASE)
#define SCUM_UART    ((SCUM_UART_TypeDef *)SCUM_UART_BASE)

#define SCUM_DMA_RF_RX_ADDR *(__IO uint8_t **)(SCUM_DMA_BASE + 0x14)
#define SCUM_ADC_START      *(__IO uint32_t *)(SCUM_ADC_START_BASE)
#define SCUM_ADC_DATA       *(__IO uint32_t *)(SCUM_ADC_DATA_BASE)

#define SCUM_GPIO_INPUT  *(__IO uint32_t *)(SCUM_GPIO_BASE + 0x000000)
#define SCUM_GPIO_OUTPUT *(__IO uint32_t *)(SCUM_GPIO_BASE + 0x040000)

#define SCUM_ANALOG_CFG_REG_0     *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x000000)
#define SCUM_ANALOG_CFG_REG_1     *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x040000)
#define SCUM_ANALOG_CFG_REG_2     *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x080000)
#define SCUM_ANALOG_CFG_REG_3     *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x0C0000)
#define SCUM_ANALOG_CFG_REG_4     *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x100000)
#define SCUM_ANALOG_CFG_REG_5     *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x140000)  // contains 2.4 GHz divider control, see bucket_o_functions/divProgram()
#define SCUM_ANALOG_CFG_REG_6     *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x180000)  // contains 2.4 GHz divider control, see bucket_o_functions/divProgram()
#define SCUM_ANALOG_CFG_REG_7     *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x1C0000)  // contains 2.4 GHz oscillator control, see bucket_o_functions/LC_freqchange
#define SCUM_ANALOG_CFG_REG_8     *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x200000)  // contains 2.4 GHz oscillator control, see bucket_o_functions/LC_freqchange
#define SCUM_ANALOG_CFG_REG_9     *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x240000)
#define SCUM_ANALOG_CFG_REG_10    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x280000)
#define SCUM_ANALOG_CFG_REG_11    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x2C0000)  // contains control bits for the arbitrary TX FIFO, apparently
#define SCUM_ANALOG_CFG_REG_12    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x300000)
#define SCUM_ANALOG_CFG_REG_13    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x340000)
#define SCUM_ANALOG_CFG_REG_14    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x380000)
#define SCUM_ANALOG_CFG_REG_15    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x3C0000)
#define SCUM_ANALOG_CFG_REG_16    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x400000)
#define SCUM_ANALOG_CFG_REG_17    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x440000)
#define SCUM_ANALOG_CFG_REG_18    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x480000)
#define SCUM_ANALOG_CFG_REG_19    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x4C0000)
#define SCUM_ANALOG_CFG_REG_20    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x500000)
#define SCUM_ANALOG_CFG_REG_21    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x540000)
#define SCUM_ANALOG_CFG_REG_22    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x580000)
#define SCUM_ANALOG_CFG_REG_23    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x5C0000)
#define SCUM_ANALOG_CFG_REG_24    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x600000)
#define SCUM_ANALOG_CFG_REG_25    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x640000)
#define SCUM_ANALOG_CFG_REG_26    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x680000)
#define SCUM_ANALOG_CFG_REG_27    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x6C0000)
#define SCUM_ANALOG_CFG_REG_28    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x700000)
#define SCUM_ANALOG_CFG_REG_29    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x740000)
#define SCUM_ANALOG_CFG_REG_30    *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x780000)
#define SCUM_ANALOG_CFG_LO_ADDR   *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x1C0000)
#define SCUM_ANALOG_CFG_LO_ADDR_2 *(__IO uint32_t *)(SCUM_ANALOG_CFG_BASE + 0x200000)

#ifdef __cplusplus
}
#endif

#endif /* SCUM_H */
