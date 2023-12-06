#[Derive(Clone, Copy)]
pub struct Rule {
    pub action: Action,
    pub options: Vec<Options>,
    pub ident: Vec<u8>,
    pub target: Option<Vec<u8>>,
    pub cmd: Option<Vec<u8>>,
    pub cmd_args: Option<Vec<Vec<u8>>>,
    pub envlist: HashMap<Vec<u8>, Vec<u8>>,
}
