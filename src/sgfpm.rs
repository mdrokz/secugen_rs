use std::{ffi::c_void, mem::MaybeUninit};

use crate::{
    CreateSGFPMObject, SGDeviceInfoParam, SGFDxDeviceName_SG_DEV_AUTO,
    SGFDxErrorCode_SGFDX_ERROR_NONE, SGFDxErrorCode_SGFDX_ERROR_WRONG_IMAGE,
    SGFPM_EnableCheckOfFingerLiveness, SGFPM_EnableSmartCapture, SGFPM_GetDeviceInfo,
    SGFPM_GetImage, SGFPM_GetMatchingScore, SGFPM_Init, SGFPM_MatchTemplate, SGFPM_OpenDevice,
    SGFPM_SetBrightness, SGFPM_SetFakeDetectionLevel, SGFPM,
};

#[derive(Debug, Clone, Copy)]
pub struct DeviceInfo {
    pub image_width: u64,
    pub image_height: u64,
    pub com_speed: u64,
    pub com_port: u64,
}

#[derive(Debug)]

pub struct FPM {
    sgfpm: *mut SGFPM,
    device_info: Option<DeviceInfo>,
}

impl FPM {
    pub fn new() -> Self {
        Self {
            sgfpm: std::ptr::null_mut(),
            device_info: None,
        }
    }

    pub fn capture_image(&mut self) -> Result<Vec<u8>, String> {
        if let Some(device_info) = self.device_info {
            let buffer_size = (device_info.image_width * device_info.image_height) as usize;
            let mut buffer = vec![0xCCu8; buffer_size];

            let err = unsafe { SGFPM_GetImage(self.sgfpm as *mut c_void, buffer.as_mut_ptr()) };

            if err != SGFDxErrorCode_SGFDX_ERROR_NONE.into() {
                if err != SGFDxErrorCode_SGFDX_ERROR_WRONG_IMAGE.into() {
                    return Err(format!("GetImage(): Failed : ErrorCode = {}", err));
                }
            }

            return Ok(buffer);
        } else {
            return Err("Device not initialized".to_string());
        }
    }

    pub fn match_template(
        &mut self,
        mut min_template_1: Vec<u8>,
        mut min_template_2: Vec<u8>,
        secu_level: u32,
    ) -> Result<bool, String> {
        let mut matched = 0;

        unsafe {
            let err = SGFPM_MatchTemplate(
                self.sgfpm as *mut c_void,
                min_template_1.as_mut_ptr(),
                min_template_2.as_mut_ptr(),
                secu_level.into(),
                &mut matched as *mut i32,
            );

            if err != SGFDxErrorCode_SGFDX_ERROR_NONE.into() {
                return Err(format!("MatchTemplate(): Failed : ErrorCode = {}", err));
            }
        }

        Ok(matched == 1)
    }

    pub fn get_matching_score(
        &mut self,
        mut min_template_1: Vec<u8>,
        mut min_template_2: Vec<u8>,
    ) -> Result<u64, String> {
        let mut score = 0;
        unsafe {
            let err = SGFPM_GetMatchingScore(
                self.sgfpm as *mut c_void,
                min_template_1.as_mut_ptr(),
                min_template_2.as_mut_ptr(),
                &mut score as *mut u64,
            );

            if err != SGFDxErrorCode_SGFDX_ERROR_NONE.into() {
                return Err(format!("MatchTemplate(): Failed : ErrorCode = {}", err));
            }
        }

        Ok(score)
    }

    pub fn init_device(
        &mut self,
        brightness: Option<u64>,
        fake_detection_level: Option<i32>,
        smart_capture: Option<bool>,
        check_finger_liveness: Option<i32>,
    ) -> Result<bool, String> {
        let mut sgfpm = MaybeUninit::uninit();

        unsafe {
            CreateSGFPMObject(sgfpm.as_mut_ptr());
        }

        self.sgfpm = unsafe { sgfpm.assume_init() };

        if self.sgfpm.is_null() {
            return Err("Failed to create SGFPM object".to_string());
        }

        unsafe {
            let err = SGFPM_Init(
                self.sgfpm as *mut c_void,
                SGFDxDeviceName_SG_DEV_AUTO.into(),
            );

            if err != SGFDxErrorCode_SGFDX_ERROR_NONE.into() {
                return Err(format!("Init: Failed : ErrorCode = {}", err));
            }

            let err = SGFPM_OpenDevice(self.sgfpm as *mut c_void, 0);

            if err != SGFDxErrorCode_SGFDX_ERROR_NONE.into() {
                return Err(format!("OpenDevice: Failed : ErrorCode = {}", err));
            }

            if let Some(v) = brightness {
                SGFPM_SetBrightness(self.sgfpm as *mut c_void, v);
            } else {
                SGFPM_SetBrightness(self.sgfpm as *mut c_void, 50);
            }

            if let Some(v) = fake_detection_level {
                SGFPM_SetFakeDetectionLevel(self.sgfpm as *mut c_void, v);
            }

            if let Some(v) = smart_capture {
                SGFPM_EnableSmartCapture(self.sgfpm as *mut c_void, v);
            }

            if let Some(v) = check_finger_liveness {
                SGFPM_EnableCheckOfFingerLiveness(self.sgfpm as *mut c_void, v);
            }

            let mut p_info: MaybeUninit<SGDeviceInfoParam> = MaybeUninit::uninit();
            let err = SGFPM_GetDeviceInfo(self.sgfpm as *mut c_void, p_info.as_mut_ptr());

            if err != SGFDxErrorCode_SGFDX_ERROR_NONE.into() {
                return Err(format!("GetDeviceInfo: Failed : ErrorCode = {}", err));
            }

            let p_info = p_info.assume_init();

            let device_info = DeviceInfo {
                image_width: p_info.ImageWidth,
                image_height: p_info.ImageHeight,
                com_speed: p_info.ComSpeed,
                com_port: p_info.ComPort,
            };

            self.device_info = Some(device_info);

            Ok(true)
        }
    }
}
