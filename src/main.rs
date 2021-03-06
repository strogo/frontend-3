#![feature(proc_macro)]

extern crate cairo;
extern crate docopt;
extern crate env_logger;
extern crate glib;
extern crate gdk;
extern crate gdk_sys;
extern crate gtk;
extern crate gtk_sys;
#[macro_use]
extern crate log;
#[macro_use]
extern crate relm;
extern crate relm_core;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;
extern crate redpitaya_scpi;
#[macro_use]
extern crate serde_derive;
extern crate serde;

mod application;
mod color;
mod scales;
mod widget;

use relm::Widget;
use scales::Scales;

static USAGE: &'static str = "Usage: yellow-pitaya <addr>";

#[derive(Deserialize)]
struct Args
{
    arg_addr: String,
}

fn main() {
    env_logger::init()
        .unwrap();

    let docopt = match ::docopt::Docopt::new(USAGE) {
        Ok(docopt) => docopt,
        Err(error) => error.exit(),
    };

    let args: Args = match docopt.deserialize() {
        Ok(args) => args,
        Err(e) => e.exit(),
    };

    let redpitaya = ::redpitaya_scpi::Redpitaya::new(args.arg_addr);

    application::Widget::run(redpitaya)
        .unwrap();
}
