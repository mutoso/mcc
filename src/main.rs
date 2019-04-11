/*
    Copyright © 2019 Alastair Feille

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

extern crate clap;

#[macro_use]
extern crate log;
extern crate env_logger;

mod tokenizer;

use clap::{App, Arg};
use env_logger::fmt::Color;
use log::Level;
use std::{fs, io::Write};

fn main()
{
    env_logger::builder().format(|buf, record| {
                             // pick the right color
                             let mut style = buf.style();
                             let style = match record.level()
                             {
                                 Level::Trace => style.set_color(Color::Black).set_intense(true),
                                 Level::Debug => style.set_color(Color::White),
                                 Level::Info => style.set_color(Color::Green),
                                 Level::Warn => style.set_color(Color::Yellow),
                                 Level::Error => style.set_color(Color::Red).set_bold(true)
                             };
                             writeln!(buf,
                                      "[{} {} {}::{}:{}] {}",
                                      buf.timestamp(),
                                      style.value(record.level()),
                                      record.module_path().unwrap_or("UNKNOWN"),
                                      record.file()
                                            .unwrap_or("UNKNOWN")
                                            .split('/')
                                            .last()
                                            .unwrap_or("UNKNOWN"),
                                      record.line().unwrap_or(0),
                                      record.args())
                         })
                         .init();

    // parse arguments
    let args = App::new("mcc").version(env!("CARGO_PKG_VERSION"))
                              .about("mutos compiler for C")
                              .author(env!("CARGO_PKG_AUTHORS"))
                              .arg(Arg::with_name("file").help("input files")
                                                         .required(true)
                                                         .min_values(1))
                              .get_matches();

    // go through each file
    let files = args.values_of_lossy("file").unwrap();
    for file in files
    {
        let input_text = fs::read_to_string(file).unwrap();
        let tokens = tokenizer::run(&input_text);
        for token in tokens
        {
            println!("{}: {:?}", token.name, token.lexeme);
        }
    }
}
