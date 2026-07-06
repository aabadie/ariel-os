/* SCuM memory layout.
 *
 * The chip has no flash: the firmware is loaded into the 64 KiB program
 * memory at each boot by the scum-programmer tool. The load address is the
 * execution address, so the program memory is used as the FLASH region.
 */
MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 64K
    RAM   : ORIGIN = 0x20000000, LENGTH = 64K
}
