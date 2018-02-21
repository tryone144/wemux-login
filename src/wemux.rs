// This file is part auf the wemux-login package.
//
// (c) 2018 Bernd Busse
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.
//

use std::process::{Command, ExitStatus, Stdio};
use std::fmt;

use std::fmt::Display;

use std::os::unix::process::ExitStatusExt;


const WEMUX_EXE: &'static str = "wemux";


pub struct WemuxError {
    msg: String,
    code: i32,
}

impl WemuxError {
    pub fn new(msg: &str, code: i32) -> Self {
        WemuxError {
            msg: msg.to_string(),
            code: code
        }
    }

    pub fn msg(&self) -> String {
        self.msg.to_string()
    }

    pub fn code(&self) -> i32 {
        self.code
    }
}


#[derive(Debug, PartialEq)]
pub enum Mode {
    ListSessions,
    Mirror,
    Pair,
    Rogue,
}

impl Display for Mode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Mode::ListSessions => write!(fmt, "list"),
            &Mode::Mirror => write!(fmt, "mirror"),
            &Mode::Pair => write!(fmt, "pair"),
            &Mode::Rogue => write!(fmt, "rogue"),
        }
    }
}


fn new_command(replace_io: bool) -> Command {
    let mut cmd  = Command::new(WEMUX_EXE);
    cmd.env_remove("SSH_CONNECTION")
        .env_remove("SSH_CLIENT")
        .env_remove("SSH_TTY");

    if replace_io {
        cmd.stdout(Stdio::inherit())
            .stdin(Stdio::inherit());
    } else {
        cmd.stdout(Stdio::piped())
            .stdin(Stdio::null());
    }

    cmd
}

fn child_failed(status: &ExitStatus) -> Option<WemuxError> {
    match status.code() {
        Some(0) => None,
        Some(code) => Some(WemuxError::new(
                format!("'{}' failed with {}", WEMUX_EXE, status).as_ref(),
                code)),
        None => Some(WemuxError::new(
                format!("'{}' was killed by signal {}", WEMUX_EXE,
                        status.signal().unwrap_or(0)).as_ref(),
                9)),
    }
}

fn mirror() -> Result<(), WemuxError> {
    let mut child = new_command(true)
        .arg("client").arg("mirror")
        .spawn().expect(format!("Cannot execute '{} client pair'", WEMUX_EXE).as_ref());

    let status = child.wait()
        .expect(format!("Cannot wait for '{}' to finish", WEMUX_EXE).as_ref());

    if let Some(err) = child_failed(&status) {
        return Err(err);
    }

    Ok(())
}

fn pair() -> Result<(), WemuxError> {
    let mut child = new_command(true)
        .arg("client").arg("pair")
        .spawn().expect(format!("Cannot execute '{} client pair'", WEMUX_EXE).as_ref());

    let status = child.wait()
        .expect(format!("Cannot wait for '{}' to finish", WEMUX_EXE).as_ref());

    if let Some(err) = child_failed(&status) {
        return Err(err);
    }

    Ok(())
}

pub fn list_sessions() -> Result<(), WemuxError> {
    let child = new_command(false)
        .arg("client").arg("list")
        .output().expect(format!("Cannot execute '{} list'", WEMUX_EXE).as_ref());

    if let Some(err) = child_failed(&child.status) {
        return Err(err);
    }

    println!("{}", String::from_utf8_lossy(&child.stdout));
    Ok(())
}

pub fn run(mode: Mode, session: Option<String>) -> Result<(), WemuxError> {
    match session {
        None => {
            println!("Connect to default session in {} mode...", mode);
            let mut child = new_command(false)
                .arg("client").arg("reset")
                .output().expect(format!("Cannot execute '{} reset'", WEMUX_EXE).as_ref());
            if let Some(err) = child_failed(&child.status) {
                return Err(err);
            }
        },
        Some(ref sess) => {
            println!("Connect to session '{}' in {} mode...", sess, mode);
            let mut child = new_command(false)
                .arg("client").arg("join").arg(sess)
                .output().expect(format!("Cannot execute '{} join {}'", WEMUX_EXE, sess).as_ref());
            if let Some(err) = child_failed(&child.status) {
                return Err(err);
            }
        },
    }

    match mode {
        Mode::Mirror => mirror(),
        Mode::Pair => pair(),
        _ => unreachable!("Got unsupported command"),
    }
}
