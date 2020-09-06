extern crate nix;
#[macro_use]
extern crate error_chain;
extern crate gtk;
extern crate gio;
use std::ffi::*;
error_chain!{
    foreign_links{
        Nix(nix::Error);
        Io(::std::io::Error) ;
    }
}
static mut curs: Option<CString> = None;
pub fn main(arguments: Vec<&str>) -> std::result::Result<(),crate::Error>{
    match arguments[0]{
        "run" => {
            let a = ["";0];
            let a: [&CStr;0] = unsafe{std::mem::transmute(a)};
            sandbox::setup_sandbox(arguments[1]);
            nix::unistd::execv(&CString::new(arguments[2]).unwrap(),&a);
        },
        "init" => {
            let a = ["";0];
            let a: [&CStr;0] = unsafe{std::mem::transmute(a)};
            init::init(arguments[1].contains("x"))?;
            nix::unistd::execv(&CString::new("/sbin/init").unwrap(),&a)?;
        },
        _ => panic!("not supported")
    };
    Ok(())
}
pub mod sandbox;
pub mod init;