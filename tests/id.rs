use embedded_hal_mock::eh1::spi::{Mock as SPIMock, Transaction as SPITransaction};
use ina229::INA229;

#[test]
fn read_manufacturer_id_works() {
    // Arrange
    let spi_expectations = [
        SPITransaction::transaction_start(),
        SPITransaction::write_vec(vec![0xF9]),
        SPITransaction::read_vec(vec![0x54, 0x49]),
        SPITransaction::transaction_end(),
    ];
    let spi = SPIMock::new(&spi_expectations);
    let mut ina229 = INA229::new(spi);

    // Act
    let id = ina229.manufacturer_id().expect("id to be returned");

    // Assert
    let mut spi = ina229.release();
    assert_eq!(id, 0x5449);
    spi.done();
}

#[test]
fn read_device_id_works() {
    // Arrange
    let spi_expectations = [
        SPITransaction::transaction_start(),
        SPITransaction::write_vec(vec![0xFD]),
        SPITransaction::read_vec(vec![0x22, 0x91]),
        SPITransaction::transaction_end(),
    ];
    let spi = SPIMock::new(&spi_expectations);
    let mut ina229 = INA229::new(spi);

    // Act
    let id = ina229.device_id().expect("id to be returned");

    // Assert
    let mut spi = ina229.release();
    assert_eq!(id, 0x2291);
    spi.done();
}
