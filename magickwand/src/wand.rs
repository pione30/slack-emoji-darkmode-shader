use magickwand_bindgen;
use magickwand_bindgen::{
    DestroyMagickWand, MagickReadImageFile, MagickWand, MagickWandGenesis, MagickWandTerminus,
    MagickWriteImageFile, NewMagickWand,
};

use magickwand_bindgen::MagickBooleanType_MagickFalse as MagickFalse;
// use magickwand_bindgen::MagickBooleanType_MagickTrue as MagickTrue;

use std::ffi::CString;
use std::sync::Once;

pub struct File {
    ptr: *mut magickwand_bindgen::FILE,
}

impl File {
    pub fn new(filename: &str, mode: &str) -> Self {
        let c_filename = CString::new(filename).expect("Cstring::new filename");
        let c_mode = CString::new(mode).expect("Cstring::new mode");
        let ptr = unsafe { magickwand_bindgen::fopen(c_filename.as_ptr(), c_mode.as_ptr()) };
        File { ptr }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            magickwand_bindgen::fclose(self.ptr);
        }
    }
}

static GENESIS: Once = Once::new();

fn magick_wand_genesis() {
    GENESIS.call_once(|| unsafe {
        MagickWandGenesis();
    });
}

// TODO: automatically called after the all of Wand resources are dropped
pub fn magick_wand_terminus() {
    unsafe {
        MagickWandTerminus();
    }
}

pub struct Wand {
    ptr: *mut MagickWand,
}

impl Wand {
    pub fn new() -> Self {
        magick_wand_genesis();
        let ptr = unsafe { NewMagickWand() };
        Wand { ptr }
    }

    pub fn magick_read_image_file(&self, file: &File) {
        let status = unsafe { MagickReadImageFile(self.ptr, file.ptr) };

        if status == MagickFalse {
            panic!("Magick read image file failed");
        }
    }

    pub fn magick_write_image_file(&self, file: &File) {
        let status = unsafe { MagickWriteImageFile(self.ptr, file.ptr) };

        if status == MagickFalse {
            panic!("Magick write image file failed");
        }
    }
}

impl Default for Wand {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Wand {
    fn drop(&mut self) {
        unsafe {
            self.ptr = DestroyMagickWand(self.ptr);
        }
    }
}
