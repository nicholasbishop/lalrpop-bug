use std::env;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let mut config = lalrpop::Configuration::new();
    config.set_out_dir(out_dir);
    config.set_in_dir("src");
    config.process_file("src/grammar.lalrpop").unwrap();
}
