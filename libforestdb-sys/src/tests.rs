extern crate libc;

use std::ptr::{null, null_mut};
use std::os::raw::c_void;
use std::ffi::CStr;
use std::str;
use std::mem;
use self::libc::free;
use super::*;

#[test]
fn test_binding() {
    unsafe {
        let mut fhandle: *mut fdb_file_handle = null_mut();
        let mut kvhandle: *mut fdb_kvs_handle = null_mut();
        let mut status;
        let config = fdb_get_default_config();
        let kvs_config = fdb_get_default_kvs_config();

        status = fdb_init(&config);
        assert_eq!(status, fdb_status::FDB_RESULT_SUCCESS);
        status = fdb_open(&mut fhandle, "/tmp/db_filename".as_ptr() as *const _, &config);
        assert_eq!(status, fdb_status::FDB_RESULT_SUCCESS);
        status = fdb_kvs_open(fhandle, &mut kvhandle, null(), &kvs_config);
        assert_eq!(status, fdb_status::FDB_RESULT_SUCCESS);

        status = fdb_set_kv(kvhandle,
                            "foo".as_ptr() as *const _, 3,
                            "bar".as_ptr() as *const _, 3);
        assert_eq!(status, fdb_status::FDB_RESULT_SUCCESS);
        status = fdb_commit(fhandle, FDB_COMMIT_OPT::FDB_COMMIT_NORMAL as u8);
        assert_eq!(status, fdb_status::FDB_RESULT_SUCCESS);
        let mut value: *mut c_void = null_mut();
        let mut value_len: usize = 0;
        status = fdb_get_kv(kvhandle, "foo".as_ptr() as *const _, 3,
                            &mut value, &mut value_len);
        assert_eq!(status, fdb_status::FDB_RESULT_SUCCESS);
        let v = str::from_utf8(CStr::from_ptr(mem::transmute(value)).to_bytes()).unwrap();
        assert_eq!(v, "bar");
        free(value as *mut _);

        status = fdb_close(fhandle);
        assert_eq!(status, fdb_status::FDB_RESULT_SUCCESS);
        status = fdb_shutdown();
        assert_eq!(status, fdb_status::FDB_RESULT_SUCCESS);
    }
}
