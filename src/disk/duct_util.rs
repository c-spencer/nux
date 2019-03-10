use std::error::Error;
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

    pub fn to_expr(self) -> Expr {
        Expr::new(self.to_duct())
    }

    pub fn to_expr_with_wait(self, wait: u64) -> Expr {
        Expr::new_with_wait(self.to_duct(), wait)
    }

    pub fn as_expr<F>(self, f: F) -> Expr
    where
        F: Fn(duct::Expression) -> duct::Expression,
    {
        Expr::new(f(self.to_duct()))
    }

    pub fn to_duct(self) -> duct::Expression {
        duct::cmd(self.program, self.args)
    }
}

pub struct Expr {
    expr: duct::Expression,
    wait: u64,
}

impl Expr {
    pub fn new(expr: duct::Expression) -> Expr {
        Expr {
            expr: expr,
            wait: 50,
        }
    }

    pub fn new_with_wait(expr: duct::Expression, wait: u64) -> Expr {
        Expr {
            expr: expr,
            wait: wait,
        }
    }

    pub fn exec(&self, execute: bool, pb: &indicatif::ProgressBar) {
        pb.println(format!("{:?}", self.expr));

        if execute {
            match self.expr.stdout_capture().stderr_capture().run() {
                Ok(result) => {
                    let stdout = String::from_utf8(result.stdout).unwrap();
                    if stdout.len() > 0 {
                        pb.println(stdout);
                    }

                    let stderr = String::from_utf8(result.stderr).unwrap();
                    if stderr.len() > 0 {
                        pb.println(stderr);
                    }
                }

                Err(err) => {
                    println!("err {:#?}", err);
                    println!("source {:#?}", err.source());
                    println!("description {:#?}", err.description());
                    println!("{:#?}", self.expr);
                    panic!("Aborting.");
                }
            }
        }

        if self.wait > 0 {
            std::thread::sleep(std::time::Duration::from_millis(self.wait));
        }
    }
}
