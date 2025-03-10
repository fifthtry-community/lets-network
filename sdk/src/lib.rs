#![allow(clippy::derive_partial_eq_without_eq, clippy::get_first)]
#![deny(unused_crate_dependencies)]
#![warn(clippy::used_underscore_binding)]
#![forbid(unsafe_code)]

// -- record contact:
// caption name:
// optional string title: ;;  their job title
// optional lets-network.organisation org: ;; their company/school name name
// string url: ;; e.g., https://amitu.com/ranedk.com
// optional string domain:
// optional string profile-picture:
//
//
// -- record organisation:
// caption name:
// string url: ;; e.g., https://amitu.com/ranedk.com
// optional string domain: ;; e.g., google.com
// optional string logo:

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Contact {
    pub name: String,
    pub title: Option<String>,
    pub org: Option<Organisation>,
    pub url: String,
    pub domain: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Organisation {
    pub name: String,
    pub url: String,
    pub domain: Option<String>,
    pub logo: Option<String>,
}
