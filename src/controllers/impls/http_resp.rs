use std::collections::HashMap;

pub trait ExecFunc<T> {
    // fn new(&self) -> &self;
    fn get_code(&self) -> i32;
    fn set_code(&mut self, code: i32);
    fn get_data(&self) -> T;
    fn set_data(&mut self, data: T);
    fn get_info(&self) -> String;
    fn set_info(&mut self, info: String);
    fn done(&self) -> Self;
}

pub struct HttpResp<T> {
    info: String,
    data: T,
    code: i32,
}

impl<T> ExecFunc<T> for HttpResp<T> {
    // fn new(&self) -> &self {
    //     &self
    // }

    ///获取code
    fn get_code(&self) -> i32 {
        self.code.clone()
    }

    ///获取code
    fn set_code(&mut self, code: i32) {
        self.code = code;
    }

    ///获取code
    fn get_data(&self) -> T {
        self.data.clone()
    }

    ///获取code
    fn set_data(&mut self, data: T) {
        self.data = data;
    }

    ///获取code
    fn get_info(&self) -> String {
        self.info.clone()
    }

    ///获取code
    fn set_info(&mut self, info: String) {
        self.info = info;
    }

    ///获取code
    fn done(&self) -> Self {
        Self {
            info: self.info.clone(),
            data: self.data.clone(),
            code: self.code.clone(),
        }
    }
}
