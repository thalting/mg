use cli::Options;

mod cli;
mod ffi;
mod visitor;

fn main() {
    let options: Options = argh::from_env();

    options.into_visitor().symlink_files()
}
