//usage

fn usage() {
    eprintln!("usage: doas [-nSs] [-a style] [-C config] [-u user] command [args]\n");
    std::process::exit(1);
}

/*
static int parseuid(const char *s, uid_t *uid) {
	struct passwd *pw;
    pw = getpwnam(s);
	if (pw != NULL) {
		*uid = pw->pw_uid;
		return 0;
	}
	return -1;
}
*/

