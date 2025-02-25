use serial::prelude::*;
use std::{
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
    time::{Duration, Instant},
};

use crate::{app::Event, AppArgs};
pub(crate) fn begin_serial(args: &AppArgs) -> Option<(JoinHandle<()>, Receiver<Event>)> {
    let Ok(mut serial) = serial::open(&args.device) else {
        eprintln!("Failed to open serial port {}.", &args.device);
        return None;
    };

    let (tx, rx) = std::sync::mpsc::channel();

    let handle = std::thread::spawn(move || {
        if let Err(e) = run(&mut serial, tx) {
            eprintln!("Error: {e:?}");
        }
    });

    Some((handle, rx))
}

fn run(serial: &mut impl SerialPort, tx: Sender<Event>) -> std::io::Result<()> {
    serial.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud9600)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    serial.set_timeout(Duration::from_secs(5))?;
    // When the device last sent a '.', ensuring the connection is still alive.
    let mut last_heartbeat = Instant::now();
    // Used to manage the do_reset state machine, see do_reset
    let mut last = '\0';
    // Buffer to read a single u8 from the serial port at a time
    let mut buf = [0; 1];
    loop {
        // Ensure the serial port is still alive, and if not, send a disconnect
        // event to the app.
        if last_heartbeat.elapsed().as_secs() > serial.timeout().as_secs() {
            _ = tx.send(Event::Disconnected);
            break;
        }

        let count = serial.read(&mut buf[..])?;
        if count == 0 {
            continue;
        }

        // If true, the board was reset. The app will
        // release all paddles as if nothing was pressed.
        if do_reset(buf[0], &mut last) {
            _ = tx.send(Event::Reset);
            continue;
        }

        _ = tx.send(match buf[0] {
            b'{' => Event::LeftPress,
            b'}' => Event::RightPress,
            b'[' => Event::LeftRelease,
            b']' => Event::RightRelease,
            b'.' => {
                last_heartbeat = Instant::now();
                continue;
            }
            _ => continue,
        });
    }

    Ok(())
}

// Implements a minimal state machine that
// will determine if the serial connection
// reset, aka the hardware button to reset
// was pressed. This will cause all paddles
// to be set to unpressed.
// The full sequence read over serial is:
// '\n', '\n', 'R', 'E', 'A', 'D', 'Y', '\n', '\n'
// However, this state machine replaces ambiguous
// bytes with '\r' and '0' so the position is always
// known. '\0' is considered the initial value for the
// `last` value.
fn do_reset(read: u8, last: &mut char) -> bool {
    match (&last, read) {
        ('\0', b'\n') => *last = '\n',

        ('\n', b'\n') => *last = '\r',

        ('\r', b'R') => *last = 'R',

        ('R', b'E') => *last = 'E',

        ('E', b'A') => *last = 'A',
        ('A', b'D') => *last = 'D',
        ('D', b'Y') => *last = 'Y',
        ('Y', b'\n') => *last = '0',
        ('0', b'\n') => {
            *last = '\0';
            return true;
        }
        _ => {}
    }
    false
}
