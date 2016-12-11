extern crate libforestdb_sys as ffi;
use std::ptr::{null, null_mut};
use std::result::Result;
use std::os::raw::c_void;
use std::mem;


type FdbResult<T> = Result<T, ffi::fdb_status>;

macro_rules! try_fdb {
    ($buf_func_res:expr) => ({
        let res = $buf_func_res;
        if res != ffi::fdb_status::FDB_RESULT_SUCCESS {
            return Err(res)
        }
    })
}

#[derive(Debug)]
pub struct Fdb {
    fhandle: *mut ffi::fdb_file_handle,
    kvhandle: *mut ffi::fdb_kvs_handle,
}

impl Fdb {
    pub fn open(db_path: &str) -> FdbResult<Self> {
        unsafe {
            let mut fdb = Fdb{
                fhandle: null_mut(),
                kvhandle: null_mut(),
            };
            let config = ffi::fdb_get_default_config();
            let kvs_config = ffi::fdb_get_default_kvs_config();

            try_fdb!(ffi::fdb_init(&config));
            try_fdb!(ffi::fdb_open(&mut fdb.fhandle, db_path.as_ptr() as *const _, &config));
            try_fdb!(ffi::fdb_kvs_open(fdb.fhandle, &mut fdb.kvhandle, null(), &kvs_config));

            Ok(fdb)
        }
    }

    pub fn get(&mut self, key: &[u8]) -> FdbResult<Option<Vec<u8>>> {
        unsafe {
            let mut value: *mut c_void = null_mut();
            let mut value_len: usize = 0;
            match ffi::fdb_get_kv(self.kvhandle, key.as_ptr() as *const _, key.len(),
                                  &mut value, &mut value_len) {
                ffi::fdb_status::FDB_RESULT_SUCCESS => (),
                ffi::fdb_status::FDB_RESULT_KEY_NOT_FOUND => return Ok(None),
                other_errs => return Err(other_errs)
            }
            Ok(Some(Vec::from_raw_parts(mem::transmute(value), value_len, value_len)))
        }
    }

    pub fn set(&mut self, key: &[u8], value: &[u8]) -> FdbResult<()> {
        unsafe {
            try_fdb!(ffi::fdb_set_kv(self.kvhandle,
                     key.as_ptr() as *const _, key.len(),
                     value.as_ptr() as *const _, value.len()));
            try_fdb!(ffi::fdb_commit(self.fhandle,
                     ffi::FDB_COMMIT_OPT::FDB_COMMIT_NORMAL as u8));
            Ok(())
        }
    }
}

impl Drop for Fdb {
    fn drop(&mut self) {
        if self.fhandle.is_null() {
            return;
        }
        unsafe {
            assert_eq!(ffi::fdb_close(self.fhandle), ffi::fdb_status::FDB_RESULT_SUCCESS);
            assert_eq!(ffi::fdb_shutdown(), ffi::fdb_status::FDB_RESULT_SUCCESS);
        }
    }
}

#[cfg(test)]
mod tests;
