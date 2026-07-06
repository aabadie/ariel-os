#include <stdio.h>
#include <stdint.h>

#include "scum.h"
#include "spi.h"

#include "helpers.h"

#define CS_PIN   15
#define CLK_PIN  14
#define DIN_PIN  13  // Used when reading data from the IMU thus a SCuM input
#define DATA_PIN 12  // Used when writing to the IMU thus a SCuM output

void spi_write(unsigned char writeByte) {

    for (uint8_t j = 7; j >= 0; j--) {
        if ((writeByte & (0x01 << j))) {
            SCUM_GPIO_OUTPUT &= ~(1 << CLK_PIN);  // clock low
            SCUM_GPIO_OUTPUT |= 1 << DATA_PIN;    // write a 1
            SCUM_GPIO_OUTPUT |= 1 << CLK_PIN;     // clock high
        } else {
            SCUM_GPIO_OUTPUT &= ~(1 << CLK_PIN);   // clock low
            SCUM_GPIO_OUTPUT &= ~(1 << DATA_PIN);  // write a 0
            SCUM_GPIO_OUTPUT |= (1 << CLK_PIN);    // clock high
        }
    }

    SCUM_GPIO_OUTPUT &= ~(1 << data_pin);  // set data out to 0
}

unsigned char spi_read(void) {
    unsigned char readByte = 0;

    SCUM_GPIO_OUTPUT &= ~(1 << CLK_PIN);  // clock low

    for (uint8_t j = 7; j >= 0; j--) {
        SCUM_GPIO_OUTPUT |= (1 << CLK_PIN);  // clock high
        readByte |= ((SCUM_GPIO_OUTPUT & (1 << DIN_PIN)) >> DIN_PIN) << j;
        SCUM_GPIO_OUTPUT &= ~(1 << CLK_PIN);  // clock low
    }

    return readByte;
}

void spi_chip_select(void) {
    // drop chip select low to select the chip
    SCUM_GPIO_OUTPUT &= ~(1 << CS_PIN);
    SCUM_GPIO_OUTPUT &= ~(1 << DATA_PIN);

    busy_wait_cycles(50);
}

void spi_chip_deselect(void) {
    // hold chip select high to deselect the chip
    SCUM_GPIO_OUTPUT |= (1 << CS_PIN);
}

void initialize_imu(void) {
    write_imu_register(0x06, 0x41);
    busy_wait_cycles(50000);
    write_imu_register(0x06, 0x01);
    busy_wait_cycles(50000);
}

unsigned int read_acc_x(void) {
    unsigned char write_byte = 0x2D;
    unsigned int acc_x = (read_imu_register(write_byte)) << 8;

    write_byte = 0x2E;
    acc_x |= read_imu_register(write_byte);

    return acc_x;
}

unsigned int read_acc_y(void) {
    unsigned char write_byte = 0x2F;
    unsigned int acc_y = (read_imu_register(write_byte)) << 8;

    write_byte = 0x30;
    acc_y |= read_imu_register(write_byte);

    return acc_y;
}

unsigned int read_acc_z(void) {
    unsigned char write_byte = 0x31;
    unsigned int acc_z = (read_imu_register(write_byte)) << 8;

    write_byte = 0x32;
    acc_z |= read_imu_register(write_byte);

    return acc_z;
}

unsigned int read_gyro_x(void) {
    unsigned char write_byte = 0x33;
    unsigned int gyro_x = (read_imu_register(write_byte)) << 8;

    write_byte = 0x34;
    gyro_x |= read_imu_register(write_byte);

    return gyro_x;
}

unsigned int read_gyro_y(void) {
    unsigned char write_byte = 0x35;
    unsigned int gyro_y = (read_imu_register(write_byte)) << 8;

    write_byte = 0x36;
    gyro_y |= read_imu_register(write_byte);

    return gyro_y;
}

unsigned int read_gyro_z(void) {
    unsigned char write_byte = 0x37;
    unsigned int gyro_z = (read_imu_register(write_byte)) << 8;

    write_byte = 0x38;
    gyro_z |= read_imu_register(write_byte);

    return gyro_z;
}

void test_imu_life(void) {
    unsigned char write_byte = 0x00;
    unsigned char read_byte = read_imu_register(write_byte);

    if (read_byte == 0xEA) {
        printf("My IMU is alive!!!\n");
    } else {
        printf("My IMU is not working :( \n");
    }
}

unsigned char read_imu_register(unsigned char reg) {
    reg &= 0x7F;
    reg |= 0x80;  // guarantee that the function input is a valid input (not
                  // necessarily a valid, and readable, register)

    spi_chip_select();                     // drop chip select
    spi_write(reg);                        // write the selected register to the port
    unsigned char read_byte = spi_read();  // clock out the bits and read them
    spi_chip_deselect();                   // raise chip select

    return read_byte;
}

void write_imu_register(unsigned char reg, unsigned char data) {
    reg &= 0x7F;  // guarantee that the function input is valid (not necessarily
                  // a valid, and readable, register)

    spi_chip_select();    // drop chip select
    spi_write(reg);       // write the selected register to the port
    spi_write(data);      // write the desired register contents
    spi_chip_deselect();  // raise chip select
}

void read_all_imu_data(imu_data_t *imu_measurement) {
    imu_measurement->acc_x.value = read_acc_x();
    imu_measurement->acc_y.value = read_acc_y();
    imu_measurement->acc_z.value = read_acc_z();
    imu_measurement->gyro_x.value = read_gyro_x();
    imu_measurement->gyro_y.value = read_gyro_y();
    imu_measurement->gyro_z.value = read_gyro_z();
}

void log_imu_data(imu_data_t *imu_measurement) {
    printf(
        "AX: %3d %3d, AY: %3d %3d, AZ: %3d %3d, GX: %3d %3d, GY: %3d %3d, GZ: "
        "%3d %3d\n",
        imu_measurement->acc_x.bytes[0], imu_measurement->acc_x.bytes[1],
        imu_measurement->acc_y.bytes[0], imu_measurement->acc_y.bytes[1],
        imu_measurement->acc_z.bytes[0], imu_measurement->acc_z.bytes[1],
        imu_measurement->gyro_x.bytes[0], imu_measurement->gyro_x.bytes[1],
        imu_measurement->gyro_y.bytes[0], imu_measurement->gyro_y.bytes[1],
        imu_measurement->gyro_z.bytes[0], imu_measurement->gyro_z.bytes[1]);
}
