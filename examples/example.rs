#![no_std]
#![no_main]

use esp32c3_hal::i2c::I2C;
use esp32c3_hal::pac::Peripherals;
use esp32c3_hal::{clock::ClockControl, timer::TimerGroup, Rtc};
use esp32c3_hal::{prelude::*, IO};
use esp_backtrace as _;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the watchdog timers. For the ESP32-C3, this includes the Super WDT,
    // the RTC WDT, and the TIMG WDTs.
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Create a new peripheral object with the described wiring
    // and standard I2C clock speed
    let mut i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio1,
        io.pins.gpio2,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    )
    .unwrap();

    let mut buffer = [0u8; 1];
    // 我们必须将地址向左移动 1 的原因与 I2C 期望 7 个地址位后跟 1 个其他位指示读取或写入操作的事实有关，但是到目前为止，我们的最后一个地址位是最右边的位所以我们想将它向左移动一位，为读写位腾出空间。
    i2c.write_read(0x1B << 1, &[0x00], &mut buffer).unwrap();

    loop {}
}
