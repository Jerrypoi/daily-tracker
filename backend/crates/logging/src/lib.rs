use chrono::{NaiveDate, Utc};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

tokio::task_local! {
    pub static LOG_ID: String;
}

pub fn current_log_id() -> String {
    LOG_ID
        .try_with(|id| id.clone())
        .unwrap_or_else(|_| "-".to_string())
}

const DEFAULT_LOG_DIR: &str = "/var/log/daily-tracker";
const FILE_PREFIX: &str = "backend";
const FILE_SUFFIX: &str = "log";
const SYMLINK_NAME: &str = "current.log";

fn dated_filename(date: NaiveDate) -> String {
    format!("{}-{}.{}", FILE_PREFIX, date.format("%Y-%m-%d"), FILE_SUFFIX)
}

fn open_dated(dir: &Path, date: NaiveDate) -> io::Result<File> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join(dated_filename(date)))
}

// Atomically point `current.log` at the dated file by creating a temp symlink
// and renaming it over the live one — avoids a window where the link is missing.
fn update_symlink(dir: &Path, date: NaiveDate) -> io::Result<()> {
    let target = dated_filename(date);
    let link = dir.join(SYMLINK_NAME);
    let tmp = dir.join(format!("{}.tmp", SYMLINK_NAME));
    let _ = fs::remove_file(&tmp);
    symlink(&target, &tmp)?;
    fs::rename(&tmp, &link)
}

struct DailyRollingFile {
    dir: PathBuf,
    state: Mutex<RollState>,
}

struct RollState {
    date: NaiveDate,
    file: File,
}

impl DailyRollingFile {
    fn new(dir: PathBuf) -> io::Result<Self> {
        fs::create_dir_all(&dir)?;
        let today = Utc::now().date_naive();
        let file = open_dated(&dir, today)?;
        update_symlink(&dir, today)?;
        Ok(Self {
            dir,
            state: Mutex::new(RollState { date: today, file }),
        })
    }
}

impl Write for DailyRollingFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let today = Utc::now().date_naive();
        let mut state = self.state.lock().unwrap();
        if state.date != today {
            state.file = open_dated(&self.dir, today)?;
            update_symlink(&self.dir, today)?;
            state.date = today;
        }
        state.file.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.state.lock().unwrap().file.flush()
    }
}

struct TeeWriter {
    stderr: io::Stderr,
    file: DailyRollingFile,
}

impl Write for TeeWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stderr.write_all(buf)?;
        self.file.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stderr.flush()?;
        self.file.flush()
    }
}

pub fn init_logging() {
    let log_dir = std::env::var("DAILY_TRACKER_LOG_DIR")
        .unwrap_or_else(|_| DEFAULT_LOG_DIR.to_string());

    let file = DailyRollingFile::new(PathBuf::from(&log_dir))
        .expect("Failed to initialize log directory");

    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            let ts = buf.timestamp();
            writeln!(
                buf,
                "[{} {} {}] [log_id={}] {}",
                ts,
                record.level(),
                record.target(),
                current_log_id(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(Box::new(TeeWriter {
            stderr: io::stderr(),
            file,
        })))
        .init();
}
