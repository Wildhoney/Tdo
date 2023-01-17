mod cli;
mod db;
mod renderer;
mod types;

pub fn main() {
    let output = cli::run();
    renderer::print(output);
}
