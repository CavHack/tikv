// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

mod test_backup;

pub fn init() {
    use std::sync::*;
    static INIT: Once = ONCE_INIT;
    INIT.call_once(|| {
        test_util::init_log_for_test();
    })
}