/*
 * Minimal printf/puts implementation for the vendored SCuM SDK code.
 *
 * The SDK calibration code prints its progress with printf/puts. Ariel OS
 * binaries are linked with rust-lld and do not carry a C library, so this
 * shim provides just enough of stdio for the compiled SDK files, writing
 * directly to the SCuM UART data register.
 *
 * Supported conversions: %c %s %p %d %i %u %x %X and their 'l' variants,
 * plus %%. Width/precision/flags are parsed and ignored, which is
 * sufficient for the format strings used by the SDK calibration code.
 */

#include <stdarg.h>
#include <stdint.h>

#define SCUM_UART_DATA (*(volatile uint32_t *)0x51000000UL)

static void uart_putc(char c) {
    SCUM_UART_DATA = (uint32_t)c;
}

static int print_str(const char *s) {
    int n = 0;
    while (*s) {
        uart_putc(*s++);
        n++;
    }
    return n;
}

static int print_unsigned(unsigned long value, unsigned base, int uppercase) {
    char buf[11]; /* 32-bit value in base 10 or 16 */
    const char *digits = uppercase ? "0123456789ABCDEF" : "0123456789abcdef";
    int pos = 0;
    int n = 0;

    do {
        buf[pos++] = digits[value % base];
        value /= base;
    } while (value != 0 && pos < (int)sizeof(buf));

    while (pos > 0) {
        uart_putc(buf[--pos]);
        n++;
    }
    return n;
}

static int print_signed(long value) {
    int n = 0;
    unsigned long mag;
    if (value < 0) {
        uart_putc('-');
        n++;
        mag = (unsigned long)-value;
    } else {
        mag = (unsigned long)value;
    }
    return n + print_unsigned(mag, 10, 0);
}

int puts(const char *s) {
    print_str(s);
    uart_putc('\n');
    return 0;
}

int putchar(int c) {
    uart_putc((char)c);
    return c;
}

int printf(const char *fmt, ...) {
    va_list ap;
    int n = 0;

    va_start(ap, fmt);
    while (*fmt) {
        if (*fmt != '%') {
            uart_putc(*fmt++);
            n++;
            continue;
        }

        fmt++; /* skip '%' */

        /* Skip flags, width and precision */
        while (*fmt == '-' || *fmt == '+' || *fmt == ' ' || *fmt == '#' ||
               *fmt == '0') {
            fmt++;
        }
        while (*fmt >= '0' && *fmt <= '9') {
            fmt++;
        }
        if (*fmt == '.') {
            fmt++;
            while (*fmt >= '0' && *fmt <= '9') {
                fmt++;
            }
        }

        int is_long = 0;
        while (*fmt == 'l') {
            is_long = 1;
            fmt++;
        }

        switch (*fmt) {
            case '%':
                uart_putc('%');
                n++;
                break;
            case 'c':
                uart_putc((char)va_arg(ap, int));
                n++;
                break;
            case 's':
                n += print_str(va_arg(ap, const char *));
                break;
            case 'p':
                n += print_str("0x");
                n += print_unsigned((unsigned long)va_arg(ap, void *), 16, 0);
                break;
            case 'd':
            case 'i':
                n += print_signed(is_long ? va_arg(ap, long)
                                          : (long)va_arg(ap, int));
                break;
            case 'u':
                n += print_unsigned(is_long ? va_arg(ap, unsigned long)
                                            : va_arg(ap, unsigned int),
                                    10, 0);
                break;
            case 'x':
            case 'X':
                n += print_unsigned(is_long ? va_arg(ap, unsigned long)
                                            : va_arg(ap, unsigned int),
                                    16, *fmt == 'X');
                break;
            case '\0':
                va_end(ap);
                return n;
            default:
                /* Unknown conversion: print it raw */
                uart_putc('%');
                uart_putc(*fmt);
                n += 2;
                break;
        }
        fmt++;
    }
    va_end(ap);

    return n;
}
