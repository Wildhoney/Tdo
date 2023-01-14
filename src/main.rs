mod actions;
mod cli;
mod renderer;
mod types;
mod utils;

pub fn main() {
    let output = cli::run();
    renderer::print(output);
}
