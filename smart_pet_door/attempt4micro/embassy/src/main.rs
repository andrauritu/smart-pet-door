#![no_std]
#![no_main]

use core::fmt;
use core::fmt::Write;
use core::str::FromStr;

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::Input;
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::PIN_14;
use embassy_rp::peripherals::UART0;
use embassy_rp::peripherals::UART1;
use embassy_rp::peripherals::{PIN_2, PIN_4, PIN_5, PWM_CH1};
use embassy_rp::pwm::{Config as PwmConfig, Pwm};
use embassy_rp::uart;
use embassy_rp::uart::BufferedUart;
use embassy_time::Duration;
use embassy_time::Timer;
use embedded_io_async::Read;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    UART0_IRQ => embassy_rp::uart::BufferedInterruptHandler<UART0>;
    UART1_IRQ => embassy_rp::uart::BufferedInterruptHandler<UART1>;

});
struct Buffer<const N: usize>([u8; N], usize);

impl<const N: usize> Write for Buffer<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let space_left = self.0.len() - self.1;
        if space_left >= s.len() {
            self.0[self.1..][..s.len()].copy_from_slice(s.as_bytes());
            self.1 += s.len();
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}

impl<const N: usize> Buffer<N> {
    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.0[..self.1]).unwrap()
    }
    fn clear(&mut self) {
        self.1 = 0;
    }
}
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut tx_buf0 = [0u8; 500];
    let mut rx_buf0 = [0u8; 500];
    let mut config0 = uart::Config::default();
    config0.baudrate = 9600; // Set baud rate to 9600
    let mut uart0 = BufferedUart::new(
        p.UART0,
        Irqs,
        p.PIN_0,
        p.PIN_1,
        &mut tx_buf0,
        &mut rx_buf0,
        config0,
    );

    let mut config1 = uart::Config::default();
    config1.baudrate = 9600;
    let mut tx_buf1 = [0u8; 500];
    let mut rx_buf1 = [0u8; 500];

    let mut uart1 = BufferedUart::new(
        p.UART1,
        Irqs,
        p.PIN_4,
        p.PIN_5,
        &mut tx_buf1,
        &mut rx_buf1,
        config1,
    );

    let pir_sensor = Input::new(p.PIN_14, Pull::None);

    let mut pwm_config: PwmConfig = Default::default();
    let min = 0x07AE;
    let mid = min + min / 2;
    let max = min * 2;
    pwm_config.top = 0x9999;
    pwm_config.compare_a = mid;
    pwm_config.divider = 64.into();

    let mut servo = Pwm::new_output_a(p.PWM_CH1, p.PIN_2, pwm_config.clone());

    let mut buf = [0u8; 500];

    /* this loop continuously checks for motion. if motion is detected, it scans for nearby beacons,
    and if it finds a beacon with a specific UUID, it checks the RSSI value and if it is less than 80,
    it unlocks the door */
    loop {
        if pir_sensor.is_high() {
            uart1
                .blocking_write(b"Motion detected, starting beacon scan...\r\n")
                .unwrap();
            let command = b"AT+DISI?\r\n"; //this is the command to scan for nearby beacons
                                           //the hm-10 was previously configured to be in master role, by sending AT+ROLE1

            uart0.blocking_write(command).unwrap();

            let mut response_buffer = Buffer([0u8; 2048], 0);

            loop {
                match uart0.read(&mut buf).await {
                    Ok(bytes_read) => {
                        if bytes_read > 0 {
                            response_buffer
                                .write_str(core::str::from_utf8(&buf[..bytes_read]).unwrap())
                                .unwrap();

                            if response_buffer.as_str().ends_with("OK+DISCE\r\n") {
                                uart1.blocking_write(b"nearby beacons:\r\n").unwrap();
                                uart1
                                    .blocking_write(response_buffer.as_str().as_bytes())
                                    .unwrap();
                                break;
                            }
                        } else {
                            uart1.blocking_write(b"read 0 bytes\r\n").unwrap();
                        }
                    }
                    Err(_) => {
                        uart1
                            .blocking_write(b"Error reading from UART1\r\n")
                            .unwrap();
                    }
                }

                Timer::after(Duration::from_millis(100)).await;
            }
            let uuid_to_find = "4C000215:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA:11111111D8:060504030201";
            /* it looks like this because i configured the beacon to send this specific UUID, thinking i would
            search for just the "AAAAAAAAAAA" part, but i ended up searching for the whole thing because i needed
            the rssi value to check the distance anyway */
            for line in response_buffer.as_str().lines() {
                if let Some((uuid, rssi_str)) = line.split_once(":-") {
                    //the rssi value is after the ":-" part at the end of the id
                    if uuid.contains(uuid_to_find) {
                        if let Ok(rssi) = i32::from_str(rssi_str.trim()) {
                            if rssi < 80 {
                                uart1
                                    .blocking_write(b"\n MY CAT IS AT THE DOOR!!\r\n")
                                    .unwrap();
                                pwm_config.compare_a = min;
                                servo.set_config(&pwm_config);
                                Timer::after(Duration::from_secs(10)).await;
                                pwm_config.compare_a = mid;
                                servo.set_config(&pwm_config);
                                Timer::after(Duration::from_secs(1)).await;
                                pwm_config.compare_a = max;
                                servo.set_config(&pwm_config);
                            } else {
                                uart1
                                    .blocking_write(b"\n My cat is nearby but not at the door.\r\n")
                                    .unwrap();
                            }
                        }
                    } else {
                        uart1.blocking_write(b"\n not my cat.\r\n").unwrap();
                    }
                }
            }
            response_buffer.clear();
            Timer::after_secs(2).await;
        } else {
            uart1.blocking_write(b"No motion detected.\r\n").unwrap();
            Timer::after(Duration::from_secs(1)).await;
        }
    }
}
