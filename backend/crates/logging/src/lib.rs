use std::io::Write;
use std::fs::OpenOptions;

/// A writer that duplicates output to both stderr and a file.
struct TeeWriter {
    stderr: std::io::Stderr,
    file: std::fs::File,
}

impl Write for TeeWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stderr.write_all(buf)?;
        self.file.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stderr.flush()?;
        self.file.flush()?;
        Ok(())
    }
}

pub fn init_logging() {
    let log_file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("server.log")
    .expect("Failed to open log file");

    env_logger::Builder::from_default_env()
    .target(env_logger::Target::Pipe(Box::new(TeeWriter {
        stderr: std::io::stderr(),
        file: log_file,
    })))
    .init();
}

