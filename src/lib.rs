use std::{mem::MaybeUninit, ffi::c_void};

use neon::prelude::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[derive(Debug, Clone, Copy)]
struct DeviceInfo {
    image_width: u64,
    image_height: u64,
    com_speed: u64,
    com_port: u64,
}


pub fn init_device<'a>(mut cx: FunctionContext<'a>) -> JsResult<'a, JsBoolean> {
    // let mut fpm = fpm.borrow_mut();
    // match fpm.init_device() {
    //     Ok(_) => Ok(cx.boolean(true)),
    //     Err(e) => cx.throw_error(e),
    // }
    // let mut fpm2 = FPM::new();

    // match fpm2.init_device() {
    //     Ok(_) => Ok(cx.boolean(true)),
    //     Err(e) => cx.throw_error(e),
    // }

    println!("Init device");

    let mut sgfpm = MaybeUninit::uninit();

    // let mut sgfpm_ptr = sgfpm.as_mut_ptr();

    unsafe {
        CreateSGFPMObject(sgfpm.as_mut_ptr());
    }

    // println!("sgfpm_ptr: {:?}", sgfpm_ptr);
    let sgfpm = unsafe { sgfpm.assume_init() };

    if sgfpm.is_null() {
        // println!("Failed to create SGFPM object");
        return cx.throw_error("Failed to create SGFPMObject");
        // return Err("Failed to create SGFPM object".to_string());
    }

    unsafe {
        println!("Init device");
        let err = SGFPM_Init(sgfpm as *mut c_void, SGFDxDeviceName_SG_DEV_AUTO.into());

        println!("err: {:?}", err);

        if err != SGFDxErrorCode_SGFDX_ERROR_NONE.into() {
            // return Err(format!("Init: Failed : ErrorCode = {}", err));
        }

        let err = SGFPM_OpenDevice(sgfpm as *mut c_void, 0);

        if err != SGFDxErrorCode_SGFDX_ERROR_NONE.into() {
            // return Err(format!("OpenDevice: Failed : ErrorCode = {}", err));
        }

        SGFPM_SetBrightness(sgfpm as *mut c_void, 50);

        let mut p_info: MaybeUninit<SGDeviceInfoParam> = MaybeUninit::uninit();
        let err = SGFPM_GetDeviceInfo(sgfpm as *mut c_void, p_info.as_mut_ptr());

        if err != SGFDxErrorCode_SGFDX_ERROR_NONE.into() {
            // return Err(format!("GetDeviceInfo: Failed : ErrorCode = {}", err));
        }

        let p_info = p_info.assume_init();

        let device_info = DeviceInfo {
            image_width: p_info.ImageWidth,
            image_height: p_info.ImageHeight,
            com_speed: p_info.ComSpeed,
            com_port: p_info.ComPort,
        };
    }

    Ok(cx.boolean(true))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("initDevice", init_device)?;

    Ok(())
}
