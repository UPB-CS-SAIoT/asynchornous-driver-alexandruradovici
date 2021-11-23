//! Hello Driver
//! Commands
//!     0 -> SUCCESS
//!     1 -> print Hello
//!

use core::cell::Cell;

use kernel::grant::Grant;
use kernel::hil::led::Led;
use kernel::hil::time::{Alarm, AlarmClient, ConvertTicks};
use kernel::process::{Error, ProcessId};
use kernel::syscall::{CommandReturn, SyscallDriver};
use kernel::{debug, ErrorCode};

pub const DRIVER_NUM: usize = 0xa0000;

pub struct Hello<'a, A: Alarm<'a>, L: Led> {
    access_grant: Grant<(), 1>,
    alarm: &'a A,
    led: &'a L,
    status: Cell<HelloStatus>,
}

#[derive(Copy, Clone)]
enum HelloStatus {
    Idle,
    Printing(usize, ProcessId), // Printing {
                                //     times: usize
                                //     process_id: ProcessId
                                // }
}

impl<'a, A: Alarm<'a>, L: Led> Hello<'a, A, L> {
    pub fn new(alarm: &'a A, access_grant: Grant<(), 1>, led: &'a L) -> Hello<'a, A, L> {
        Hello {
            alarm,
            access_grant,
            led,
            status: Cell::new(HelloStatus::Idle),
        }
    }

    fn print_hello(&self) {
        match self.status.get() {
            HelloStatus::Idle => {
                unreachable!();
            }
            HelloStatus::Printing(times, process_id) => {
                if times > 0 {
                    debug!("{} Hello", times);
                    self.status
                        .set(HelloStatus::Printing(times - 1, process_id));
                    self.alarm
                        .set_alarm(self.alarm.now(), self.alarm.ticks_from_seconds(1));
                } else {
                    let _ = self
                        .access_grant
                        .enter(process_id, |_app_memory, upcalls_table| {
                            let _ = upcalls_table.schedule_upcall(0, (0, 0, 0));
                        });
                    self.led.off();
                    self.status.set(HelloStatus::Idle);
                }
            }
        }
    }
}

impl<'a, A: Alarm<'a>, L: Led> SyscallDriver for Hello<'a, A, L> {
    fn command(
        &self,
        command_num: usize,
        r2: usize,
        _r3: usize,
        process_id: ProcessId,
    ) -> CommandReturn {
        match command_num {
            0 => CommandReturn::success(),
            1 => {
                if let HelloStatus::Idle = self.status.get() {
                    self.status.set(HelloStatus::Printing(r2, process_id));
                    self.led.on();
                    // self.status.set (HelloStatus::Printing{times: r2, process_id});
                    self.print_hello();
                    CommandReturn::success()
                } else {
                    CommandReturn::failure(ErrorCode::BUSY)
                }
            }
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }

    fn allocate_grant(&self, process_id: ProcessId) -> Result<(), Error> {
        self.access_grant
            .enter(process_id, |_app_data, _upcalls_table| {})
    }
}

impl<'a, A: Alarm<'a>, L: Led> AlarmClient for Hello<'a, A, L> {
    fn alarm(&self) {
        self.print_hello();
    }
}
