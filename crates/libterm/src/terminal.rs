use std::collections::VecDeque;
use std::io;
use std::time::Duration;

use libterm_core::event::Event;
use libterm_core::mode::Mode;
use libterm_core::op::Op;
use libterm_sys::{PollResult, RawGuard, Tty, Waker};

use crate::state::TerminalState;

pub struct Terminal {
    tty: Tty,
    guard: Option<RawGuard>,
    waker: Option<Waker>,
    state: TerminalState,
    read_buf: [u8; 4096],
    events: VecDeque<Event>,
    last_size: Option<(u16, u16)>,
}

impl Terminal {
    pub fn open() -> io::Result<Self> {
        let tty = Tty::open()?;
        let waker = Waker::new().ok();
        let last_size = tty.size().ok();
        let mut state = TerminalState::new();
        if let Some((cols, rows)) = last_size {
            state.encoder_mut().set_size(cols, rows);
        }

        #[cfg(unix)]
        if let Some(w) = &waker {
            libterm_sys::signal::register_sigwinch(w.fd());
        }

        Ok(Self {
            tty,
            guard: None,
            waker,
            state,
            read_buf: [0; 4096],
            events: VecDeque::new(),
            last_size,
        })
    }

    pub fn enable_raw_mode(&mut self) -> io::Result<()> {
        if self.guard.is_none() {
            let guard = RawGuard::enter(&self.tty)?;
            self.guard = Some(guard);
        }

        Ok(())
    }

    pub fn disable_raw_mode(&mut self) -> io::Result<()> {
        if let Some(mut guard) = self.guard.take() {
            guard.restore()?;
        }

        Ok(())
    }

    pub fn is_raw_mode(&self) -> bool {
        self.guard.is_some()
    }

    pub fn size(&self) -> io::Result<(u16, u16)> {
        self.tty.size()
    }

    pub fn waker(&self) -> Option<&Waker> {
        self.waker.as_ref()
    }

    pub fn push_mode(&mut self, mode: Mode) -> io::Result<()> {
        self.flush()?;
        let enable_bytes = self.state.modes_mut().push(mode).to_vec();
        self.state.encoder_mut().invalidate();
        self.tty.write_all(&enable_bytes)
    }

    pub fn pop_mode(&mut self) -> io::Result<()> {
        self.flush()?;
        if let Some(disable_bytes) = self.state.modes_mut().pop() {
            self.state.encoder_mut().invalidate();
            self.tty.write_all(&disable_bytes)?;
        }

        Ok(())
    }

    pub fn emit(&mut self, op: impl Op) {
        self.state.encoder_mut().emit(op);
    }

    pub fn flush(&mut self) -> io::Result<()> {
        let encoder = self.state.encoder_mut();
        if encoder.pending().is_empty() {
            return Ok(());
        }

        let result = self.tty.write_all(encoder.pending());
        match result {
            Ok(()) => encoder.commit(),
            Err(_) => encoder.invalidate(),
        }

        result
    }

    pub fn restore(&mut self) -> io::Result<()> {
        self.flush()?;

        let restore = self.state.modes_mut().restore();
        if !restore.is_empty() {
            self.tty.write_all(&restore)?;
        }

        self.disable_raw_mode()?;
        Ok(())
    }

    pub fn poll(
        &mut self,
        timeout: Option<Duration>,
    ) -> io::Result<Option<Event>> {
        if let Some(event) = self.events.pop_front() {
            return Ok(Some(event));
        }

        if libterm_sys::signal::resized()
            && let Ok(new_size) = self.tty.size()
            && self.last_size != Some(new_size)
        {
            self.last_size = Some(new_size);
            self.state.encoder_mut().set_size(new_size.0, new_size.1);
            return Ok(Some(Event::Resize {
                cols: new_size.0,
                rows: new_size.1,
            }));
        }

        let timeout_ms = match timeout {
            Some(d) => d.as_millis().min(i32::MAX as u128) as i32,
            None => -1,
        };

        match self.tty.poll(self.waker.as_ref(), timeout_ms)? {
            PollResult::Input => match self.tty.read(&mut self.read_buf) {
                Ok(0) => Ok(None),
                Ok(n) => {
                    let mut events = std::mem::take(&mut self.events);
                    events.clear();
                    self.state.input(&self.read_buf[..n], |ev| {
                        events.push_back(ev.into_owned());
                    });

                    if self.state.has_pending()
                        && matches!(
                            self.tty.poll(None, 0),
                            Ok(PollResult::Timeout)
                        )
                    {
                        self.state.timeout(|ev| {
                            events.push_back(ev.into_owned());
                        });
                    }
                    self.events = events;

                    self.did_resize();
                    Ok(self.events.pop_front())
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(None),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                    self.did_resize();
                    Ok(self.events.pop_front())
                }
                Err(e) => Err(e),
            },
            PollResult::Waker => {
                self.did_resize();
                Ok(self.events.pop_front())
            }
            PollResult::Timeout => {
                let mut events = std::mem::take(&mut self.events);
                events.clear();
                self.state.timeout(|ev| {
                    events.push_back(ev.into_owned());
                });
                self.events = events;

                self.did_resize();
                Ok(self.events.pop_front())
            }
        }
    }

    fn did_resize(&mut self) {
        if let Ok(new_size) = self.tty.size()
            && self.last_size != Some(new_size)
        {
            self.last_size = Some(new_size);
            self.state.encoder_mut().set_size(new_size.0, new_size.1);
            self.events.push_back(Event::Resize {
                cols: new_size.0,
                rows: new_size.1,
            });
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        #[cfg(unix)]
        libterm_sys::signal::unregister_sigwinch();

        let _ = self.restore();
    }
}
