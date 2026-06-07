//! Compliance functionality

use crate::error::{Result, SdkResult, TapSdkError};
use crate::sdk::is_initialized;

/// Compliance API handle.
///
/// Get an instance via `Compliance::get()` after initializing the SDK.
pub struct Compliance {
    handle: *mut tapsdk_pc_sys::ITapCompliance,
}

// The SDK exposes compliance through a process-wide singleton.
unsafe impl Send for Compliance {}
unsafe impl Sync for Compliance {}

impl Compliance {
    /// Get the compliance singleton instance.
    pub fn get() -> Option<Self> {
        if !is_initialized() {
            return None;
        }

        let handle = unsafe { tapsdk_pc_sys::TapCompliance() };

        if handle.is_null() {
            None
        } else {
            Some(Compliance { handle })
        }
    }

    /// Ensure the current user has completed real-name verification.
    ///
    /// The result will be delivered via the `ComplianceEnsureRealName` event
    /// when calling `TapSdk::run_callbacks()`.
    pub fn ensure_real_name(&self, request_id: i64) -> Result<()> {
        let result =
            unsafe { tapsdk_pc_sys::TapCompliance_AsyncEnsureRealName(self.handle, request_id) };

        check_sdk_result(result)
    }

    /// Enable anti-addiction checks.
    ///
    /// Anti-addiction action notifications are delivered via the
    /// `ComplianceActionsEvent` event.
    pub fn enable_anti_addiction(&self) -> Result<()> {
        let result = unsafe { tapsdk_pc_sys::TapCompliance_EnableAntiAddiction(self.handle) };

        check_sdk_result(result)
    }

    /// Check whether a payment amount is allowed.
    pub fn check_payment_limit(&self, amount: u32) -> Result<PaymentLimitResponse> {
        let request = tapsdk_pc_sys::TapComplianceCheckPaymentLimitRequest { amount };
        let mut response: tapsdk_pc_sys::TapComplianceCheckPaymentLimitResponse =
            unsafe { std::mem::zeroed() };

        let result = unsafe {
            tapsdk_pc_sys::TapCompliance_CheckPaymentLimit(self.handle, &request, &mut response)
        };

        check_sdk_result(result)?;

        Ok(PaymentLimitResponse {
            allow: response.allow,
            title: unsafe { crate::callback::fixed_c_char_array_to_string(&response.title) },
            description: unsafe {
                crate::callback::fixed_c_char_array_to_string(&response.description)
            },
        })
    }

    /// Submit a successful payment amount.
    pub fn submit_payment(&self, amount: u32) -> Result<()> {
        let request = tapsdk_pc_sys::TapComplianceSubmitPaymentRequest { amount };
        let result = unsafe { tapsdk_pc_sys::TapCompliance_SubmitPayment(self.handle, &request) };

        check_sdk_result(result)
    }
}

/// Payment limit check response.
#[derive(Debug, Clone)]
pub struct PaymentLimitResponse {
    pub allow: bool,
    pub title: String,
    pub description: String,
}

fn check_sdk_result(result: u32) -> Result<()> {
    match SdkResult::from(result) {
        SdkResult::Ok => Ok(()),
        result => Err(TapSdkError::SdkRequestFailed(result)),
    }
}
