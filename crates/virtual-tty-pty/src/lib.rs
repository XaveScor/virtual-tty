use libc::{self, winsize};
use std::io;
use std::os::unix::io::RawFd;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

// Re-export the core VirtualTty
pub use virtual_tty::VirtualTty;

pub struct PtyAdapter {
    virtual_tty: Arc<Mutex<VirtualTty>>,
    master_fd: Option<RawFd>,
    slave_fd: Option<RawFd>,
    reader_thread: Option<JoinHandle<()>>,
}

impl PtyAdapter {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            virtual_tty: Arc::new(Mutex::new(VirtualTty::new(width, height))),
            master_fd: None,
            slave_fd: None,
            reader_thread: None,
        }
    }

    pub fn from_virtual_tty(virtual_tty: VirtualTty) -> Self {
        Self {
            virtual_tty: Arc::new(Mutex::new(virtual_tty)),
            master_fd: None,
            slave_fd: None,
            reader_thread: None,
        }
    }

    pub fn get_virtual_tty(&self) -> Arc<Mutex<VirtualTty>> {
        self.virtual_tty.clone()
    }

    pub fn get_snapshot(&self) -> String {
        self.virtual_tty.lock().unwrap().get_snapshot()
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.virtual_tty.lock().unwrap().get_size()
    }

    fn create_pty(&mut self) -> io::Result<()> {
        if self.master_fd.is_some() {
            return Ok(());
        }

        let (width, height) = self.get_size();

        unsafe {
            let mut master: libc::c_int = 0;
            let mut slave: libc::c_int = 0;
            
            // Open a new PTY
            let result = libc::openpty(
                &mut master,
                &mut slave,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            
            if result != 0 {
                return Err(io::Error::last_os_error());
            }
            
            // Set window size
            let ws = winsize {
                ws_row: height as u16,
                ws_col: width as u16,
                ws_xpixel: 0,
                ws_ypixel: 0,
            };
            
            let _ = libc::ioctl(master, libc::TIOCSWINSZ, &ws);
            
            self.master_fd = Some(master);
            self.slave_fd = Some(slave);
        }

        Ok(())
    }

    fn start_reader_thread(&mut self) {
        if self.reader_thread.is_some() {
            return;
        }

        let master_fd = match self.master_fd {
            Some(fd) => fd,
            None => return,
        };

        let virtual_tty = self.virtual_tty.clone();

        let reader_thread = thread::spawn(move || {
            let mut read_buffer = [0u8; 4096];
            
            loop {
                let n = unsafe { libc::read(master_fd, read_buffer.as_mut_ptr() as *mut libc::c_void, read_buffer.len()) };
                match n {
                    0 => break, // EOF
                    n if n > 0 => {
                        let data = String::from_utf8_lossy(&read_buffer[..n as usize]);
                        virtual_tty.lock().unwrap().stdout_write(&data);
                    }
                    _ => break,
                }
            }
        });

        self.reader_thread = Some(reader_thread);
    }

    pub fn spawn_command(&mut self, cmd: &mut Command) -> io::Result<Child> {
        self.create_pty()?;
        self.start_reader_thread();

        let slave_fd = self.slave_fd.ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "No slave PTY")
        })?;

        // Duplicate the slave FD for stdin/stdout/stderr to avoid closing issues
        let slave_stdin = unsafe { libc::dup(slave_fd) };
        let slave_stdout = unsafe { libc::dup(slave_fd) };
        let slave_stderr = unsafe { libc::dup(slave_fd) };

        if slave_stdin < 0 || slave_stdout < 0 || slave_stderr < 0 {
            return Err(io::Error::last_os_error());
        }

        unsafe {
            use std::os::unix::io::FromRawFd;
            use std::process::Stdio;

            cmd.stdin(Stdio::from_raw_fd(slave_stdin))
                .stdout(Stdio::from_raw_fd(slave_stdout))
                .stderr(Stdio::from_raw_fd(slave_stderr));
        }

        cmd.spawn()
    }

    pub fn send_input(&mut self, input: &[u8]) -> io::Result<()> {
        let master_fd = self.master_fd.ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "No master PTY")
        })?;

        let result = unsafe { libc::write(master_fd, input.as_ptr() as *const libc::c_void, input.len()) };
        if result < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    /// Convenience method to send string input
    pub fn send_input_str(&mut self, input: &str) -> io::Result<()> {
        self.send_input(input.as_bytes())
    }

    /// Wait for any running processes to complete and reader thread to finish
    pub fn wait_for_completion(&mut self) {
        if let Some(thread) = self.reader_thread.take() {
            let _ = thread.join();
        }
    }
}

impl Drop for PtyAdapter {
    fn drop(&mut self) {
        // Close PTY file descriptors
        if let Some(fd) = self.master_fd {
            unsafe { libc::close(fd); }
        }
        if let Some(fd) = self.slave_fd {
            unsafe { libc::close(fd); }
        }
        
        // Wait for reader thread to finish
        if let Some(thread) = self.reader_thread.take() {
            let _ = thread.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pty_adapter_creation() {
        let adapter = PtyAdapter::new(80, 24);
        assert_eq!(adapter.get_size(), (80, 24));
    }

    #[test]
    fn test_from_virtual_tty() {
        let tty = VirtualTty::new(40, 10);
        let adapter = PtyAdapter::from_virtual_tty(tty);
        assert_eq!(adapter.get_size(), (40, 10));
    }

    #[test]
    fn test_pty_spawn() {
        let mut adapter = PtyAdapter::new(80, 24);
        let mut cmd = Command::new("echo");
        cmd.arg("Hello PTY");
        
        match adapter.spawn_command(&mut cmd) {
            Ok(mut child) => {
                let _ = child.wait();
                // Give reader thread time to process
                std::thread::sleep(std::time::Duration::from_millis(100));
                let snapshot = adapter.get_snapshot();
                assert!(snapshot.contains("Hello PTY"));
            }
            Err(_) => {
                // PTY might not be available in test environment
                println!("PTY test skipped - not available");
            }
        }
    }
}