#![no_std]
#![no_main]

use ruduino::Pin;
use ruduino::legacy::serial;
use ruduino::cores::current::{port};

#[no_mangle]
pub extern fn main() {
    port::B5::set_output();
    const BAUD: u64 = 9600;
    const UBRR: u16 = ((ruduino::config::CPU_FREQUENCY_HZ as u64) / (16 as u64) / (BAUD - 1) as u64 ) as u16;

    serial::Serial::new(UBRR)
	.character_size(serial::CharacterSize::EightBits)
	.mode(serial::Mode::Asynchronous)
	.parity(serial::Parity::Disabled)
	.stop_bits(serial::StopBits::OneBit)
	.configure();
    loop {                                                                           

        port::B5::set_high();

        ruduino::delay::delay_ms(1000);

        port::B5::set_low();

        ruduino::delay::delay_ms(1000);
	for &b in b"Hello world\n" {
	    serial::transmit(b);
	}
    }
}
