mod cli;
mod config;
mod db;
mod renderer;
mod types;

#[macro_use]
extern crate lazy_static;

pub fn main() {
    let output = cli::run();
    renderer::print(output);
}
