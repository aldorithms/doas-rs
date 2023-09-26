#[cfg(target_os ="linux")]
use std::os::unix::io::{RawFd, AsRawFd};

#[cfg(feature = "HAVE_INTTYPES_H")]
use libc::inttypes_h::*;

#[cfg(target_os = "freebsd")]
use libc::paths_h::*;

#[cfg(target_os ="linux")]
use libc::{uid_t,gid_t};

#[cfg(feature = "HAVE_LOGIN_CAP_H")]
mod login_cap {
    use login_cap_sys::login_cap;

    // Your code that uses login_cap goes here
}

use std::os::raw::{c_char, c_int};
use std::ptr;

#[cfg(not(feature = "HAVE_LOGIN_CAP_H"))]
mod login_cap {
    // Empty module or stubs if login_cap.h is not available
}

#[cfg(feature = "USE_BSD_AUTH")]
mod bsd_auth {
    use bsd_auth_sys::{bsd_auth, readpassphrase};

    // Your code that uses bsd_auth and readpassphrase goes here
}

#[cfg(not(feature = "USE_BSD_AUTH"))]
mod bsd_auth {
    // Empty module or stubs if USE_BSD_AUTH is not defined
}

#[cfg(feature = "USE_PAM")]
mod pam {
    #[cfg(feature = "OPENPAM")]
    mod openpam {
        use security::openpam;
        pub static mut PAMC: openpam::pam_conv = openpam::pam_conv {
            conv: openpam::openpam_ttyconv,
            data: std::ptr::null_mut(),
        };
    }

    #[cfg(feature = "LINUX_PAM")]
    mod linux_pam {
        use security::pam_misc;
        pub static mut PAMC: pam_misc::pam_conv = pam_misc::pam_conv {
            conv: pam_misc::misc_conv,
            data: std::ptr::null_mut(),
        };
    }

    #[cfg(feature = "SOLARIS_PAM")]
    mod solaris_pam {
        use security::pm_pam_conv;
        pub static mut PAMC: pm_pam_conv::pam_conv = pm_pam_conv::pam_conv {
            conv: pm_pam_conv::pam_tty_conv,
            data: std::ptr::null_mut(),
        };
    }

    pub use openpam::PAMC as pamc;
}

#[cfg(not(feature = "USE_PAM"))]
mod pam {
    pub static mut PAMC: *mut std::ffi::c_void = std::ptr::null_mut();
}

#[cfg(feature = "USE_PAM")]
mod pam {
    #[cfg(feature = "OPENPAM")]
    mod openpam {
        use security::openpam;
        pub static mut PAMC: openpam::pam_conv = openpam::pam_conv {
            conv: openpam::openpam_ttyconv,
            data: std::ptr::null_mut(),
        };
    }

    #[cfg(feature = "LINUX_PAM")]
    mod linux_pam {
        use security::pam_misc;
        pub static mut PAMC: pam_misc::pam_conv = pam_misc::pam_conv {
            conv: pam_misc::misc_conv,
            data: std::ptr::null_mut(),
        };
    }

    #[cfg(feature = "SOLARIS_PAM")]
    mod solaris_pam {
        use security::pm_pam_conv;
        pub static mut PAMC: pm_pam_conv::pam_conv = pm_pam_conv::pam_conv {
            conv: pm_pam_conv::pam_tty_conv,
            data: std::ptr::null_mut(),
        };
    }

    pub use openpam::PAMC as pamc;
}

use libc::{c_char, uid_t};
use std::ffi::CString;
use std::ptr;
use std::mem;
use std::ptr::null_mut;

extern "C" {
    fn getpwnam(name: *const c_char) -> *mut libc::passwd;
    fn free_passwd(pw: *mut libc::passwd);
    fn getgrnam(name: *const c_char) -> *mut libc::group;

}

fn main()
{
    todo!("Something")
}

fn parseuid(s: &str, uid: &mut uid_t) -> i32 {
    let name_cstring = CString::new(s).expect("Failed to create CString");
    unsafe {
        let pw = libc::getpwnam(name_cstring.as_ptr());
        if !pw.is_null() {
            *uid = (*pw).pw_uid;
            libc::free_passwd(pw);
            return 0;
        }
    }

    -1
}
fn uidcheck(s: &str, desired: uid_t) -> i32 {
    let mut uid: uid_t = 0;
    if parseuid(s, &mut uid) != 0 {
        return -1;
    }
    if uid != desired {
        return -1;
    }
    0
}
 
fn parsegid(s: &str, gid: &mut gid_t) -> i32 {
    let name_cstring = CString::new(s).expect("Failed to create CString");

    unsafe {
        let gr: *mut libc::group = getgrnam(name_cstring.as_ptr());
        if !gr.is_null() {
            *gid = (*gr).gr_gid;
            return 0;
        }
    }

    -1
}
struct Rule {
    ident: &'static str,
    action: Action,
    target: Option<&'static str>,
    cmd: Option<&'static str>,
    cmdargs: Option<&'static [&'static str]>,
}
fn match_rule(uid: uid_t,groups: &[gid_t],target: uid_t,cmd: &str,cmdargs: &[&str],r: &Rule) -> bool {
    if r.ident.starts_with(':') {
        let rgid_str = &r.ident[1..];
        let mut rgid: gid_t = 0;
        if parsegid(rgid_str, &mut rgid) == -1 {
            return 0;
        }
        if !groups.contains(&rgid) {
            return 0;
        }
    } else if uidcheck(r.ident, uid) != 0 {
        return 0;
    }

    if let Some(target_str) = r.target {
        if uidcheck(target_str, target) != 0 {
            return 0;
        }
    }

    if let Some(cmd_str) = r.cmd {
        if cmd != cmd_str {
            return 0;
        }

        if let Some(cmdargs_list) = r.cmdargs {
            for (i, expected_arg) in cmdargs_list.iter().enumerate() {
                if i >= cmdargs.len() {
                    return 0;
                }
                if cmdargs[i] != *expected_arg {
                    return 0;
                }
            }

            if cmdargs_list.len() != cmdargs.len() {
                return 0;
            }
        }
    }

    1
}

#[derive(Debug)]
enum Action {
    PERMIT,
    DENY,
    // Add more actions as needed
}

fn permit(uid: uid_t,groups: &[gid_t],target: uid_t,cmd: &str,cmdargs: &[&str],rules: &[&Rule]) -> bool {
    let mut lastr: Option<&Rule> = None;
    for rule in rules.iter() {
        if match_rule(uid, groups, target, cmd, cmdargs, rule) {
            lastr = Some(rule);
        }
    }
    if lastr.is_none() {
        return false;
    }
    lastr.unwrap().action == Action::PERMIT
}

fn parseconfig(filename: &str, checkperms: i32) -> () {
    todo!("parse configuration")
}