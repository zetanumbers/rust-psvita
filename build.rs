use std::env;

fn main() {
    let vitasdk = env::var("VITASDK").expect("cannot find VITASDK enviroment variable set");
    println!("cargo:rustc-link-search={}/arm-vita-eabi/lib", vitasdk);
}
