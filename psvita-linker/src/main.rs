use std::env;

fn main() {
    let args: Vec<_> = ld_args::args().map_iter(env::args()).collect();
    let env_vars: Vec<_> = env::vars().collect();
    dbg!(args);
    dbg!(env_vars);
    todo!()
}
