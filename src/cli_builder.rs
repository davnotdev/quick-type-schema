use super::*;

#[derive(Debug, Clone)]
pub struct CliBuilder {
    args: Vec<String>,
}

impl CliBuilder {
    pub fn new(lang: &Language) -> Self {
        let args = vec!["-l".to_owned(), lang.name().to_owned()];
        Self { args }
    }

    pub fn opt_bool(mut self, arg: &str, cond: bool) -> Self {
        if cond {
            self.args.push(arg.to_owned());
        }
        self
    }

    pub fn opt_string(mut self, arg: &str, opt: &str) -> Self {
        if !opt.is_empty() {
            self.args.push(arg.to_owned());
            self.args.push(opt.to_owned());
        }
        self
    }

    pub fn opt_enum<E: Default + PartialEq, F: FnOnce() -> &'static str>(
        mut self,
        arg: &str,
        opt: &E,
        f: F,
    ) -> Self {
        if *opt != E::default() {
            self.args.push(arg.to_owned());
            self.args.push(f().to_owned());
        }
        self
    }

    pub fn build(self) -> Vec<String> {
        self.args
    }
}
