use telecom_sys::*;
use std::ffi::CString;

pub struct Client {
    ptr: GoUintptr,
}

pub enum LogLevel {
    Error,
    Info,
    Debug,
}

impl Client {
    pub fn new(user_id: impl AsRef<str>,
               guild_id: impl AsRef<str>,
               session_id: impl AsRef<str>) -> Self {
        let c_user_id = CString::new(user_id.as_ref())
            .expect("String was bad, this should not happen!");
        let c_guild_id = CString::new(guild_id.as_ref())
            .expect("String was bad, this should not happen!");
        let c_session_id = CString::new(session_id.as_ref())
            .expect("String was bad, this should not happen!");
        let ptr = unsafe {
            telecom_create_client(c_user_id.into_raw(),
                                  c_guild_id.into_raw(),
                                  c_session_id.into_raw())
        };
        Self { ptr }
    }

    pub fn set_logging(&mut self, log_level: LogLevel) {
        match log_level {
            LogLevel::Error => unsafe {
                telecom_setup_logging(0, 0);
            },
            LogLevel::Info => unsafe {
                telecom_setup_logging(1, 0);
            },
            LogLevel::Debug => unsafe {
                telecom_setup_logging(1, 1);
            },
        }
    }

    pub fn update(&mut self, endpoint: impl AsRef<str>, token: impl AsRef<str>) {
        let c_endpoint = CString::new(endpoint.as_ref())
            .expect("String was bad, this should not happen!");
        let c_token = CString::new(token.as_ref())
            .expect("String was bad, this should not happen!");
        unsafe {
            telecom_client_update_server_info(self.ptr,
                                              c_endpoint.into_raw(),
                                              c_token.into_raw());
        }
    }

    pub fn play(&mut self, playable: Playable) {
        unsafe {
            telecom_client_play(self.ptr, playable.ptr);
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            telecom_client_destroy(self.ptr);
        }
    }
}

pub struct Playable {
    ptr: GoUintptr,
}

impl Playable {
    pub fn new(source: impl AsRef<str>) -> Self {
        let c_source = CString::new(source.as_ref())
            .expect("String was bad, this should not happen!");
        let ptr = unsafe {
            telecom_create_avconv_playable(c_source.into_raw())
        };
        Self { ptr }
    }
}

impl Drop for Playable {
    fn drop(&mut self) {
        unsafe {
            telecom_playable_destroy(self.ptr);
        }
    }
}
