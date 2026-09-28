use ariel_os_stm32_fmc_display::stm32f723e_disco;

ariel_os::hal::define_peripherals!(DisplayPeripherals {
    fmc: FMC,
    ne2: PG9,
    noe: PD4,
    nwe: PD5,
    a0: PF0,
    d0: PD14,
    d1: PD15,
    d2: PD0,
    d3: PD1,
    d4: PE7,
    d5: PE8,
    d6: PE9,
    d7: PE10,
    d8: PE11,
    d9: PE12,
    d10: PE13,
    d11: PE14,
    d12: PE15,
    d13: PD8,
    d14: PD9,
    d15: PD10,
    reset: PH7,
    backlight: PH11,
});

impl DisplayPeripherals {
    pub fn into_display_peripherals(self) -> stm32f723e_disco::Peripherals<'static> {
        stm32f723e_disco::Peripherals {
            fmc: self.fmc,
            ne2: self.ne2,
            noe: self.noe,
            nwe: self.nwe,
            a0: self.a0,
            d0: self.d0,
            d1: self.d1,
            d2: self.d2,
            d3: self.d3,
            d4: self.d4,
            d5: self.d5,
            d6: self.d6,
            d7: self.d7,
            d8: self.d8,
            d9: self.d9,
            d10: self.d10,
            d11: self.d11,
            d12: self.d12,
            d13: self.d13,
            d14: self.d14,
            d15: self.d15,
            reset: self.reset,
            backlight: self.backlight,
        }
    }
}
