use embedded_hal_mock::eh1::spi::{Mock as SPIMock, Transaction as SPITransaction};
use ina229::INA229;

#[test]
fn read_temperature_raw_works() {
    // Arrange
    let spi_expectations = [
        SPITransaction::transaction_start(),
        SPITransaction::write_vec(vec![0x19]),
        SPITransaction::read_vec(vec![0x0C, 0x80]),
        SPITransaction::transaction_end(),
    ];
    let spi = SPIMock::new(&spi_expectations);
    let mut ina229 = INA229::new(spi);

    // Act
    let reading = ina229.temperature_raw().expect("reading to be returned");

    // Assert
    let mut spi = ina229.release();
    assert_eq!(reading, 3200);
    spi.done();
}

#[test]
fn read_temperature_millidegrees_celsius_works() {
    // Arrange
    let spi_expectations = [
        SPITransaction::transaction_start(),
        SPITransaction::write_vec(vec![0x19]),
        SPITransaction::read_vec(vec![0x0C, 0x80]),
        SPITransaction::transaction_end(),
    ];
    let spi = SPIMock::new(&spi_expectations);
    let mut ina229 = INA229::new(spi);

    // Act
    let reading = ina229
        .temperature_millidegrees_celsius()
        .expect("reading to be returned");

    // Assert
    let mut spi = ina229.release();
    assert_eq!(reading, 25000.0);
    spi.done();
}
