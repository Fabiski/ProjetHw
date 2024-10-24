#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;


const PORTB: *mut u8 = 0x25 as *mut u8; // PORTB address
const DDRB: *mut u8 = 0x24 as *mut u8;  // DDRB address
const PORTD: *mut u8 = 0x2B as *mut u8; // PORTD address
const DDRD: *mut u8 = 0x2A as *mut u8;  // DDRD address
const PINB: *mut u8 = 0x23 as *mut u8;  //PINB address
const PIND: *mut u8 = 0x29 as *mut u8;  //PIND address
static mut SOURCE: u8 = 0b00000001;
static mut CHOICED: *mut u8 = 0x2A as *mut u8;
static mut CHOICEP: *mut u8 = 0x2B as *mut u8;
static mut CHOICEPIN: *mut u8 = 0x29 as *mut u8;

//https://tenor.com/view/rust-femboy-rust-femboy-programming-rust-programming-gif-27321790

#[entry]
fn main() -> ! {

    let num=2;
    unsafe{
    match num{
     0=>{SOURCE = 0b00000001;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},   
     1=>{SOURCE = 0b00000010;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},
     2=>{SOURCE = 0b00000100;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},
     3=>{SOURCE = 0b00001000;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},
     4=>{SOURCE = 0b00010000;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},
     5=>{SOURCE = 0b00100000;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},
     6=>{SOURCE = 0b01000000;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},
     7=>{SOURCE = 0b10000000;CHOICED = DDRD; CHOICEP = PORTD; CHOICEPIN = PIND;},
     8=>{SOURCE = 0b00000001;CHOICED = DDRB; CHOICEP = PORTB; CHOICEPIN = PINB;},
     9=>{SOURCE = 0b00000010;CHOICED = DDRB; CHOICEP = PORTB; CHOICEPIN = PINB;},
     10=>{SOURCE = 0b00000100;CHOICED = DDRB; CHOICEP = PORTB; CHOICEPIN = PINB;},
     11=>{SOURCE = 0b00001000;CHOICED = DDRB; CHOICEP = PORTB; CHOICEPIN = PINB;},
     12=>{SOURCE = 0b00010000;CHOICED = DDRB; CHOICEP = PORTB; CHOICEPIN = PINB;},
     13=>{SOURCE = 0b00100000;CHOICED = DDRB; CHOICEP = PORTB; CHOICEPIN = PINB;},
     _=>panic!("Raté"),
    }
    }
    loop {
        unsafe {
            core::ptr::write_volatile(CHOICED,SOURCE);
            //core::ptr::write_volatile(CHOICEP, SOURCE);
            for _ in 0..1_000_000 
            {
                core::ptr::write_volatile(CHOICEP, SOURCE);
            }
            for _ in 0..1_000_000
            {
                core::ptr::write_volatile(CHOICEP, 0b00000000);
            }
        }
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}