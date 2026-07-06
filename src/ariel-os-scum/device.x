/* SCuM interrupt handlers.
 *
 * All handlers default to the cortex-m-rt DefaultHandler. OPTICAL_SFD and
 * EXT_GPIO8_ACTIVEHIGH are not listed here: they are defined in the crate
 * (src/vectors.rs) and forward to the SDK optical calibration handler.
 */
PROVIDE(UART = DefaultHandler);
PROVIDE(EXT_GPIO3_ACTIVEHIGH_DEBOUNCED = DefaultHandler);
PROVIDE(EXT_OPTICAL_IRQ_IN = DefaultHandler);
PROVIDE(ADC = DefaultHandler);
PROVIDE(RF = DefaultHandler);
PROVIDE(RFTIMER = DefaultHandler);
PROVIDE(RAWCHIPS_STARTVAL = DefaultHandler);
PROVIDE(RAWCHIPS_32 = DefaultHandler);
PROVIDE(EXT_GPIO9_ACTIVELOW = DefaultHandler);
PROVIDE(EXT_GPIO10_ACTIVELOW = DefaultHandler);
