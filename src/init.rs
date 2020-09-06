use nix::{unistd::*};
use std::ffi::*;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
pub fn init(startx: bool) -> crate::Result<()>{
    let a = ["";0];
    let a: [&CStr;0] = unsafe{std::mem::transmute(a)};
    let startx_cmd = CString::new("/.dog/links/startx").unwrap();
    if startx {match fork()?{ForkResult::Child => unsafe{std::mem::transmute(execv(&startx_cmd, &a)?)},_ => {}}};
    println!("loading...");
if startx{match fork()?{
    ForkResult::Child => {
        let application = Application::new(
            Some("com.SmalltalkLand.dog"),
            Default::default(),
        ).expect("failed to initialize GTK application");
    
        application.connect_activate(|app| {
            let window = ApplicationWindow::new(app);
            window.set_title("Dog Utilities");
            window.set_default_size(350, 70);
    
           /* let button = Button::with_label("shutdown");
            button.connect_clicked(|_| {
                execv(&CString::new("/.dog/links/kill").unwrap(),&["-9","1"].iter().map(|x| -> &CStr{&CString::new(*x).unwrap()}).collect::<Vec<&CStr>>()).unwrap();
            });
            window.add(&button);*/
    
            window.show_all();
        });
    
        application.run(&[]);
    }
    _ => {}
}};
Ok(())
}