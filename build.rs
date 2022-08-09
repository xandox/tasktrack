use clap::CommandFactory;
use clap_complete::{generate_to, shells::*};

include!("src/cli.rs");

fn main() {
    let mut app = CLI::into_app();
    let binname = "tasktrack";
    app.set_bin_name(binname);
    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions/");
    std::fs::create_dir_all(&outdir).unwrap();
    generate_to(Bash, &mut app, binname, &outdir).unwrap();
    generate_to(Fish, &mut app, binname, &outdir).unwrap();
    generate_to(Zsh, &mut app, binname, &outdir).unwrap();
    generate_to(PowerShell, &mut app, binname, &outdir).unwrap();
    generate_to(Elvish, &mut app, binname, &outdir).unwrap();
}
