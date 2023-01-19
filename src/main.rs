mod cli;
mod config;
mod db;
mod renderer;
mod types;

pub fn main() {
    let output = cli::run();
    renderer::print(output);
}
