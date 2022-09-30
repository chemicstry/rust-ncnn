use crate::datareader::DataReader;
use ncnn_bind::*;
use std::ffi::CString;
use std::marker::PhantomData;

pub struct Net {
    ptr: ncnn_net_t,
}

impl Net {
    pub fn new() -> Net {
        let ptr;
        unsafe {
            ptr = ncnn_net_create();
        }
        Net { ptr }
    }

    pub fn set_option(&mut self, opt: &crate::option::Option) {
        unsafe {
            ncnn_net_set_option(self.ptr, opt.ptr());
        }
    }

    pub fn load_param(&mut self, path: &str) -> anyhow::Result<()> {
        let c_str = CString::new(path).unwrap();
        if unsafe { ncnn_net_load_param(self.ptr, c_str.as_ptr()) } != 0 {
            anyhow::bail!("Error loading params {}", path);
        } else {
            Ok(())
        }
    }

    pub fn load_model(&mut self, path: &str) -> anyhow::Result<()> {
        let c_str = CString::new(path).unwrap();
        if unsafe { ncnn_net_load_model(self.ptr, c_str.as_ptr()) } != 0 {
            anyhow::bail!("Error loading model {}", path);
        } else {
            Ok(())
        }
    }

    pub fn load_model_datareader(&mut self, dr: &DataReader) -> anyhow::Result<()> {
        if unsafe { ncnn_net_load_model_datareader(self.ptr, dr.ptr()) } != 0 {
            anyhow::bail!("Error loading model from datareader");
        } else {
            Ok(())
        }
    }

    pub fn create_extractor(&mut self) -> Extractor<'_> {
        let ptr;
        unsafe {
            ptr = ncnn_extractor_create(self.ptr);
        }
        Extractor::from_ptr(ptr)
    }
}

impl Drop for Net {
    fn drop(&mut self) {
        unsafe {
            ncnn_net_destroy(self.ptr);
        }
    }
}

pub struct Extractor<'a> {
    ptr: ncnn_extractor_t,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Extractor<'a> {
    fn from_ptr(ptr: ncnn_extractor_t) -> Self {
        Self {
            ptr,
            _phantom: PhantomData::default(),
        }
    }

    pub fn set_option(&mut self, opt: &crate::option::Option) {
        unsafe { ncnn_extractor_set_option(self.ptr, opt.ptr()) };
    }

    pub fn input(&mut self, name: &str, mat: &'a crate::mat::Mat) -> anyhow::Result<()> {
        let c_str = CString::new(name).unwrap();
        if unsafe { ncnn_extractor_input(self.ptr, c_str.as_ptr(), mat.ptr()) } != 0 {
            anyhow::bail!("Error setting input for layer `{}`", name);
        } else {
            Ok(())
        }
    }

    pub fn extract(self, name: &str, mat: &mut crate::mat::Mat) -> anyhow::Result<()> {
        let c_str = CString::new(name).unwrap();
        if unsafe { ncnn_extractor_extract(self.ptr, c_str.as_ptr(), mat.mut_ptr()) } != 0 {
            anyhow::bail!("Error running extract on layer `{}`", name);
        } else {
            Ok(())
        }
    }
}

impl<'a> Drop for Extractor<'a> {
    fn drop(&mut self) {
        unsafe {
            ncnn_extractor_destroy(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn load_not_exist_model() {
        use crate::net::*;
        let mut net = Net::new();
        net.load_param("not_exist.param")
            .expect_err("Expected param to be not found");
    }
}
