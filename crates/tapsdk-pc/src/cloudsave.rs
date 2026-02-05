//! Cloud save functionality

use std::ffi::CString;
use std::path::Path;

use crate::error::{CloudSaveResult, Result, TapSdkError};
use crate::sdk::is_initialized;

/// Cloud save API handle
///
/// This struct provides access to cloud save functionality.
/// Get an instance via `CloudSave::get()`.
pub struct CloudSave {
    handle: *mut tapsdk_pc_sys::ITapCloudSave,
}

// The ITapCloudSave pointer is thread-safe according to the SDK documentation
unsafe impl Send for CloudSave {}
unsafe impl Sync for CloudSave {}

impl CloudSave {
    /// Get the cloud save singleton instance
    ///
    /// # Returns
    /// A `CloudSave` instance, or `None` if the SDK is not initialized
    pub fn get() -> Option<Self> {
        if !is_initialized() {
            return None;
        }

        let handle = unsafe { tapsdk_pc_sys::TapCloudSave() };

        if handle.is_null() {
            None
        } else {
            Some(CloudSave { handle })
        }
    }

    /// Request the list of cloud saves
    ///
    /// The result will be delivered via the `CloudSaveList` event
    /// when calling `TapSdk::run_callbacks()`.
    ///
    /// # Arguments
    /// * `request_id` - A unique ID to identify this request in the callback
    pub fn list(&self, request_id: i64) -> Result<()> {
        let result = unsafe { tapsdk_pc_sys::TapCloudSave_AsyncList(self.handle, request_id) };

        check_cloudsave_result(result)
    }

    /// Create a new cloud save
    ///
    /// The result will be delivered via the `CloudSaveCreate` event
    /// when calling `TapSdk::run_callbacks()`.
    ///
    /// # Arguments
    /// * `request_id` - A unique ID to identify this request in the callback
    /// * `request` - The create request parameters
    pub fn create(&self, request_id: i64, request: &CreateSaveRequest) -> Result<()> {
        let name_c = CString::new(request.name.as_str())?;
        let summary_c = CString::new(request.summary.as_str())?;
        let extra_c = request
            .extra
            .as_ref()
            .map(|s| CString::new(s.as_str()))
            .transpose()?;
        let data_path_c = CString::new(request.data_file_path.to_string_lossy().as_ref())?;
        let cover_path_c = request
            .cover_file_path
            .as_ref()
            .map(|p| CString::new(p.to_string_lossy().as_ref()))
            .transpose()?;

        let raw_request = tapsdk_pc_sys::TapCloudSaveCreateRequest {
            name: name_c.as_ptr(),
            summary: summary_c.as_ptr(),
            extra: extra_c
                .as_ref()
                .map(|s| s.as_ptr())
                .unwrap_or(std::ptr::null()),
            playtime: request.playtime,
            data_file_path: data_path_c.as_ptr(),
            cover_file_path: cover_path_c
                .as_ref()
                .map(|s| s.as_ptr())
                .unwrap_or(std::ptr::null()),
            __bindgen_padding_0: Default::default(),
        };

        let result = unsafe {
            tapsdk_pc_sys::TapCloudSave_AsyncCreate(self.handle, request_id, &raw_request)
        };

        check_cloudsave_result(result)
    }

    /// Update an existing cloud save
    ///
    /// The result will be delivered via the `CloudSaveUpdate` event
    /// when calling `TapSdk::run_callbacks()`.
    ///
    /// # Arguments
    /// * `request_id` - A unique ID to identify this request in the callback
    /// * `request` - The update request parameters
    pub fn update(&self, request_id: i64, request: &UpdateSaveRequest) -> Result<()> {
        let uuid_c = CString::new(request.uuid.as_str())?;
        let name_c = CString::new(request.name.as_str())?;
        let summary_c = CString::new(request.summary.as_str())?;
        let extra_c = request
            .extra
            .as_ref()
            .map(|s| CString::new(s.as_str()))
            .transpose()?;
        let data_path_c = CString::new(request.data_file_path.to_string_lossy().as_ref())?;
        let cover_path_c = request
            .cover_file_path
            .as_ref()
            .map(|p| CString::new(p.to_string_lossy().as_ref()))
            .transpose()?;

        let raw_request = tapsdk_pc_sys::TapCloudSaveUpdateRequest {
            uuid: uuid_c.as_ptr(),
            name: name_c.as_ptr(),
            summary: summary_c.as_ptr(),
            extra: extra_c
                .as_ref()
                .map(|s| s.as_ptr())
                .unwrap_or(std::ptr::null()),
            playtime: request.playtime,
            data_file_path: data_path_c.as_ptr(),
            cover_file_path: cover_path_c
                .as_ref()
                .map(|s| s.as_ptr())
                .unwrap_or(std::ptr::null()),
            __bindgen_padding_0: Default::default(),
        };

        let result = unsafe {
            tapsdk_pc_sys::TapCloudSave_AsyncUpdate(self.handle, request_id, &raw_request)
        };

        check_cloudsave_result(result)
    }

    /// Delete a cloud save
    ///
    /// The result will be delivered via the `CloudSaveDelete` event
    /// when calling `TapSdk::run_callbacks()`.
    ///
    /// # Arguments
    /// * `request_id` - A unique ID to identify this request in the callback
    /// * `uuid` - The unique ID of the cloud save to delete
    pub fn delete(&self, request_id: i64, uuid: &str) -> Result<()> {
        let uuid_c = CString::new(uuid)?;

        let result = unsafe {
            tapsdk_pc_sys::TapCloudSave_AsyncDelete(self.handle, request_id, uuid_c.as_ptr())
        };

        check_cloudsave_result(result)
    }

    /// Get the data file for a cloud save
    ///
    /// The result will be delivered via the `CloudSaveGetData` event
    /// when calling `TapSdk::run_callbacks()`.
    ///
    /// # Arguments
    /// * `request_id` - A unique ID to identify this request in the callback
    /// * `uuid` - The unique ID of the cloud save
    /// * `file_id` - The file ID of the cloud save (from CloudSaveInfo)
    pub fn get_data(&self, request_id: i64, uuid: &str, file_id: &str) -> Result<()> {
        let uuid_c = CString::new(uuid)?;
        let file_id_c = CString::new(file_id)?;

        let raw_request = tapsdk_pc_sys::TapCloudSaveGetFileRequest {
            uuid: uuid_c.as_ptr(),
            file_id: file_id_c.as_ptr(),
        };

        let result = unsafe {
            tapsdk_pc_sys::TapCloudSave_AsyncGetData(self.handle, request_id, &raw_request)
        };

        check_cloudsave_result(result)
    }

    /// Get the cover image for a cloud save
    ///
    /// The result will be delivered via the `CloudSaveGetCover` event
    /// when calling `TapSdk::run_callbacks()`.
    ///
    /// # Arguments
    /// * `request_id` - A unique ID to identify this request in the callback
    /// * `uuid` - The unique ID of the cloud save
    /// * `file_id` - The file ID of the cloud save (from CloudSaveInfo)
    pub fn get_cover(&self, request_id: i64, uuid: &str, file_id: &str) -> Result<()> {
        let uuid_c = CString::new(uuid)?;
        let file_id_c = CString::new(file_id)?;

        let raw_request = tapsdk_pc_sys::TapCloudSaveGetFileRequest {
            uuid: uuid_c.as_ptr(),
            file_id: file_id_c.as_ptr(),
        };

        let result = unsafe {
            tapsdk_pc_sys::TapCloudSave_AsyncGetCover(self.handle, request_id, &raw_request)
        };

        check_cloudsave_result(result)
    }
}

/// Request parameters for creating a cloud save
#[derive(Debug, Clone)]
pub struct CreateSaveRequest {
    /// Save name (max 60 bytes, no Chinese characters)
    pub name: String,
    /// Save description (max 500 bytes)
    pub summary: String,
    /// Developer-defined extra data (max 1000 bytes, optional)
    pub extra: Option<String>,
    /// Game playtime in seconds
    pub playtime: u32,
    /// Path to the save data file (max 10MB)
    pub data_file_path: Box<Path>,
    /// Path to the cover image file (max 512KB, optional)
    pub cover_file_path: Option<Box<Path>>,
}

/// Request parameters for updating a cloud save
#[derive(Debug, Clone)]
pub struct UpdateSaveRequest {
    /// UUID of the cloud save to update
    pub uuid: String,
    /// Save name (max 60 bytes, no Chinese characters)
    pub name: String,
    /// Save description (max 500 bytes)
    pub summary: String,
    /// Developer-defined extra data (max 1000 bytes, optional)
    pub extra: Option<String>,
    /// Game playtime in seconds
    pub playtime: u32,
    /// Path to the save data file (max 10MB)
    pub data_file_path: Box<Path>,
    /// Path to the cover image file (max 512KB, optional)
    pub cover_file_path: Option<Box<Path>>,
}

/// Convert a CloudSaveResult to a Result
fn check_cloudsave_result(result: u32) -> Result<()> {
    let cloud_result = CloudSaveResult::from(result);

    match cloud_result {
        CloudSaveResult::Ok => Ok(()),
        _ => Err(TapSdkError::CloudSaveRequestFailed(cloud_result)),
    }
}
