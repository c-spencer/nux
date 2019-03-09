use std::ffi::{OsStr, OsString};

pub struct Cmd {
    program: OsString,
    args: Vec<OsString>,
}

impl Cmd {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Cmd {
        Cmd {
            program: program.as_ref().to_os_string(),
            args: vec![],
        }
    }

    pub fn arg<S: AsRef<OsStr>>(mut self, arg: S) -> Cmd {
        self.args.push(arg.as_ref().to_os_string());
        self
    }

    pub fn opt<S: AsRef<OsStr>>(self, arg1: S, arg2: S) -> Cmd {
        self.arg(arg1).arg(arg2)
    }

    pub fn args<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(mut self, args: I) -> Cmd {
        for item in args {
            self.args.push(item.as_ref().to_os_string());
        }

        self
    }

    pub fn to_expr(self) -> duct::Expression {
        duct::cmd(self.program, self.args)
    }
}
