#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;

#[cfg(feature = "arduino")]
pub mod usart {
    const UBRR0H: *mut u8 = 0xC5 as *mut u8; 
    const UBRR0L: *mut u8 = 0xC4 as *mut u8; 
    const UCSR0A: *mut u8 = 0xC0 as *mut u8; 
    const UCSR0B: *mut u8 = 0xC1 as *mut u8; 
    const UCSR0C: *mut u8 = 0xC2 as *mut u8; 
    const UDR0: *mut u8 = 0xC6 as *mut u8;   

    // Initializes the USART
    pub fn init_usart(baud_rate: u32) {
        unsafe {
            let ubrr = (16_000_000 / (16 * baud_rate)) - 1;
            
            core::ptr::write_volatile(UBRR0H, (ubrr >> 8) as u8);
            core::ptr::write_volatile(UBRR0L, ubrr as u8);
            
            core::ptr::write_volatile(UCSR0B, 0b00011000);
            
            core::ptr::write_volatile(UCSR0C, 0b00000110);
        }
    }

    // Transmits using USART.
    pub fn transmit(data: u8) {
        unsafe {
            // Wait
            while (core::ptr::read_volatile(UCSR0A) & 0b00100000) == 0 {}
            
            // Write the data to the USART 
            core::ptr::write_volatile(UDR0, data);
        }
    }

    // Receives data using USART.
    pub fn receive() -> u8 {
        unsafe {
            // Wait
            while (core::ptr::read_volatile(UCSR0A) & 0b10000000) == 0 {}
            
            // Read and return the received byte from the USART
            core::ptr::read_volatile(UDR0)
        }
    }
}
