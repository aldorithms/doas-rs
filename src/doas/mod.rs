mod rule;
pub fn usage() {
    eprintln!("usage: doas [-nSs] [-a style] [-C config] [-u user] command [args]\n");
    std::process::exit(1);
}