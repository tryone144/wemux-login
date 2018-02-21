// This file is part auf the wemux-login package.
//
// (c) 2018 Bernd Busse
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.
//

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate docopt;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::process;

mod cli;
mod wemux;


fn main() {
    let args = cli::parse()
        .unwrap_or_else(|e| process::exit(e));
    let mode = args.mode().unwrap_or(wemux::Mode::Mirror);
    let session = args.session();

    if mode == wemux::Mode::ListSessions {
        wemux::list_sessions().map_err(|e| {
            eprintln!("Error: {}.", e.msg());
            e.code()
        }).unwrap_or_else(|e| process::exit(e));
    } else {
        wemux::run(mode, session).map_err(|e| {
            eprintln!("Error: {}.", e.msg());
            e.code()
        }).unwrap_or_else(|e| process::exit(e));
        println!("Wemux session closed.")
    }
}
