use embedded_hal_mock::eh1::spi::{Mock as SPIMock, Transaction as SPITransaction};
use ina229::{Configuration, INA229};

#[test]
fn read_shunt_calibration_works() {
    // Arrange
    let spi_expectations = [
        SPITransaction::transaction_start(),
        SPITransaction::write_vec(vec![0x09]),
        SPITransaction::read_vec(vec![0x10, 0x00]),
        SPITransaction::transaction_end(),
    ];
    let spi = SPIMock::new(&spi_expectations);
    let mut ina229 = INA229::new(spi);

    // Act
    let reading = ina229.shunt_calibration().expect("reading to be returned");

    // Assert
    let mut spi = ina229.release();
    assert_eq!(reading, 4096);
    spi.done();
}

#[test]
fn write_shunt_calibration_works() {
    // Arrange
    let spi_expectations = [
        SPITransaction::transaction_start(),
        SPITransaction::write_vec(vec![0x08]),
        SPITransaction::write_vec(vec![0x10, 0x00]),
        SPITransaction::transaction_end(),
    ];
    let spi = SPIMock::new(&spi_expectations);

    let mut ina229 = INA229::new(spi);

    // Act
    ina229
        .set_shunt_calibration(4096)
        .expect("write to succeed");

    // Assert
    let mut spi = ina229.release();
    spi.done();
}

#[test]
fn calibrate_works() {
    // Arrange
    let spi_expectations = [
        SPITransaction::transaction_start(),
        SPITransaction::write_vec(vec![0x00]),
        SPITransaction::write_vec(vec![0x00, 0x00]),
        SPITransaction::transaction_end(),
        SPITransaction::transaction_start(),
        SPITransaction::write_vec(vec![0x08]),
        SPITransaction::write_vec(vec![0x0F, 0xD2]),
        SPITransaction::transaction_end(),
    ];
    let spi = SPIMock::new(&spi_expectations);
    let mut ina229 = INA229::new(spi);
    ina229
        .set_configuration(Configuration::from_bits_truncate(0))
        .expect("config to be set");

    // Act
    ina229
        .calibrate(0.0162, 10.0)
        .expect("calibration to succeed");

    // Assert
    let mut spi = ina229.release();
    spi.done();
}
