extern crate libc;

use libc::{pthread_self, pthread_getname_np, pthread_setname_np};
use std::ffi::{CString, CStr, OsStr};
use std::io::{self, Error};

fn main() {
    let lalala = "la la la la la la la123456789012345678901234567890 la la la l63";
    println!("{:?}.len() = {}", lalala, lalala.len());
    let _original = getname().expect("getname default");
    setname(lalala).expect("setname");

    let new = getname().expect("getname changed");
    assert_eq!(lalala, new);

    /*unsafe {
        libc::getchar();
    }*/
}

fn getname() -> std::io::Result<String> {
    unsafe {
        let my_id = pthread_self();
        let mut name = [0i8; 256];
        let ret = pthread_getname_np(my_id, name.as_mut_ptr(), name.len());

        let name = CStr::from_ptr(name.as_ptr())
            .to_string_lossy()
            .to_owned()
            .into_owned();

        println!("{} => {:?}", ret, name);

        if ret == 0 {
            Ok(name)
        } else {
            Err(Error::last_os_error())
        }
    }
}

#[cfg(target_os = "macos")]
fn setname(new: &str) -> io::Result<()> {
    unsafe {
        let ret = pthread_setname_np(new.as_ptr() as *const i8);
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}

#[cfg(target_os = "windows")]
fn setname(new: &str) -> io::Result<()> {

}

#[cfg(other)]
fn setname(new: &str) -> io::Result<()> {
    unsafe {
        let my_id = pthread_self();
        let ret = pthread_setname_np(my_id, new.as_ptr() as *const i8);
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}