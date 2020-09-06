use nix:: {unistd::*,sched::{*}};
use std::{fs::File,io::{self, BufRead,Write},os::unix::io::{RawFd,FromRawFd},ffi::*};
use flate2::read::GzDecoder;
use tar::Archive;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
pub fn setup_sandbox(source: &str) -> Result<(),crate::Error>{
    let path = source;

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    let path = format!("/tmp/{}",thread_rng()
    .sample_iter(&Alphanumeric)
    .take(8)
    .collect::<String>());
    archive.unpack(path.clone())?;
    let path_to_use: &str = &path;
    let (pa,pb) = pipe()?;
    let (pd,pc) = pipe()?;
    dup2(pa,202)?;
    dup2(pc,203)?;
    match fork()?{ForkResult::Child => return match (|x: crate::Result<()>|x)({
        let fd = unsafe { File::from_raw_fd(pd) };
        let fb = unsafe { File::from_raw_fd(pb) };
        'main: loop{
            let mut q: Vec<Box<dyn FnMut (File) -> Result<(bool),crate::Error>>> = Vec::new();
            for line in (io::BufReader::new(fd.try_clone()?)).lines(){
                if let Err(_) = line{continue;};
                let line = line.unwrap();
                if line.starts_with("echo "){
                    writeln!(fb.try_clone()?,"{}",&line[5..]);
                    continue;
                };
                q.push(Box::new(move |fb|{let mut w = line.split_whitespace();match w.next(){
                    Some("exec") => {
                        let prog = match w.next(){Some(v) => v,None => return Ok(false)};
                        match fork()?{ForkResult::Child => {
                            let p2: &CStr = &CString::new(prog).unwrap();
                            let mut _sa = Vec::new();
                            execv(p2, &{w.for_each(|x|{let s = CString::new(x).unwrap(); _sa.push(s); ()});_sa.iter().map(|x| -> &CStr{&x})}.collect::<Vec<&CStr>>());
                        },_ => {}};
                        Ok(false)
                    },
                    _ => Ok(false)
                }}));
            };
            for item in q.iter_mut(){
                let v = item(fb.try_clone()?);
                let v = match v{Ok(vv) => vv,Err(e) => continue 'main};
                if v{
                    continue 'main;
                }
            }
        };
        Ok(())
    }){
        Ok(v) => Ok(v),
        Err(err) => panic!("Error")
    },_ => {}};
    chroot(path_to_use)?;
    unshare( CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWNET | CloneFlags::CLONE_NEWIPC | CloneFlags::CLONE_NEWCGROUP | CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWUSER)?;
    println!("in {}",path);
    Ok(())
}