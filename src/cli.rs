// This file is part auf the wemux-login package.
//
// (c) 2018 Bernd Busse
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.
//

use docopt::Docopt;
use serde::de;

use regex::Regex;

use ::wemux;


const CLI_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

const CLI_USAGE: &'static str = "
wemux-login.

Usage:
  wemux-login (list | help)
  wemux-login (mirror | pair | rogue) [SESSION]
  wemux-login -c <command>
  wemux-login (-h | --help)
  wemux-login (-v | --version)

Commands:
  help              Show this screen.
  list              List remote sessions.
  mirror [SESSION]  Connect as view-only client (default).
  pair [SESSION]    Connect as a normal client.
  rogue [SESSION]   Connect as a rogue client (experimental).

Options:
  -c <command>      SSH-compatible invocation.
  -h --help         Show this screen.
  -v --version      Show version info.
";


#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CliArgs {
    cmd_help: bool,
    cmd_list: bool,
    cmd_mirror: bool,
    cmd_pair: bool,
    cmd_rogue: bool,
    arg_SESSION: Option<String>,
    #[serde(rename = "flag_c")]
    arg_command: Option<CliCommand>,
}

impl CliArgs {
    pub fn mode(&self) -> Option<wemux::Mode> {
        self.arg_command.as_ref()
            .map(|a| match a {
                &CliCommand::List => Some(wemux::Mode::ListSessions),
                &CliCommand::Mirror(_) => Some(wemux::Mode::Mirror),
                &CliCommand::Pair(_) => Some(wemux::Mode::Pair),
                &CliCommand::Rogue(_) => Some(wemux::Mode::Rogue),
                _ => None,
            }).and_then(|o| o)
            .or_else(|| if self.cmd_list {
                Some(wemux::Mode::ListSessions)
            } else if self.cmd_mirror {
                Some(wemux::Mode::Mirror)
            } else if self.cmd_pair {
                Some(wemux::Mode::Pair)
            } else if self.cmd_rogue {
                Some(wemux::Mode::Rogue)
            } else {
                None
            })
    }

    pub fn session(&self) -> Option<String> {
        self.arg_SESSION.as_ref()
            .map(|s| s.to_string())
            .or_else(|| match self.arg_command {
                Some(CliCommand::Mirror(ref sess)) => sess.as_ref().map(|s| s.to_string()),
                Some(CliCommand::Pair(ref sess)) => sess.as_ref().map(|s| s.to_string()),
                Some(CliCommand::Rogue(ref sess)) => sess.as_ref().map(|s| s.to_string()),
                _ => None,
            })
    }
}


#[derive(Debug)]
enum CliCommand {
    Help,
    List,
    Mirror(Option<String>),
    Pair(Option<String>),
    Rogue(Option<String>),
    Unknown(String),
}

impl<'de> de::Deserialize<'de> for CliCommand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: de::Deserializer<'de> {
        lazy_static! {
            static ref RE_CMD: Regex = Regex::new(r"^(mirror|pair|rogue)(\s+([\w\-]+))?\s*$").unwrap();
        }
        let command = match String::deserialize(deserializer)?.as_ref() {
            "help" => CliCommand::Help,
            "list" => CliCommand::List,
            cmd => {
                RE_CMD.captures(cmd).as_ref().map(|c| match c.get(1).unwrap().as_str() {
                    "mirror" => CliCommand::Mirror(c.get(3).map(|s| s.as_str().to_string())),
                    "pair" => CliCommand::Pair(c.get(3).map(|s| s.as_str().to_string())),
                    "rogue" => CliCommand::Rogue(c.get(3).map(|s| s.as_str().to_string())),
                    _ => CliCommand::Unknown(cmd.to_string()),
                }).unwrap_or(CliCommand::Unknown(cmd.to_string()))
            },
        };

        Ok(command)
    }
}


pub fn parse() -> Result<CliArgs, i32> {
    let version = format!("{} version {}", "wemux-login", CLI_VERSION.unwrap_or("unknown"));
    let args: CliArgs = Docopt::new(CLI_USAGE)
        .map(|d| d.help(true).options_first(true).version(Some(version)))
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_help {
        eprintln!("{}", CLI_USAGE);
        return Err(0);
    } else if let Some(CliCommand::Help) = args.arg_command {
        eprintln!("{}", CLI_USAGE);
        return Err(0);
    } else if let Some(CliCommand::Unknown(cmd)) = args.arg_command {
        eprintln!("Unsupported command '{}'.", cmd);
        eprintln!("See '{} help' for a list of supported commands.", "wemux-login");
        return Err(1);
    }

    Ok(args)
}
