//! Utility functions and wrappers for working with iSCSI devices in SPDK.

use std::{
    fmt,
};

use snafu::{Snafu};

use crate::{
    target::iscsi::construct_iscsi_target,
    target::iscsi::ISCSI_PORTAL_GROUP_FE,
    target::iscsi::ISCSI_INITIATOR_GROUP,
    target::iscsi::unshare,
};

#[derive(Debug, Snafu)]
pub enum IscsiError {
    #[snafu(display("Failed to start iscsi target for bdev uuid {}, error {}", dev, err))]
    StartIscsi { dev: String, err: String },
}

/// Iscsi target representation.
pub struct IscsiTarget {
    bdev_uuid_str: String, // this is the bdev name (uuid)
}

impl IscsiTarget {
    /// Allocate iscsi device for the bdev and start it.
    /// When the function returns the iscsi target is ready for IO.
    pub async fn create(bdev_name: &str) -> Result<Self, IscsiError> {

        match construct_iscsi_target(bdev_name,
            ISCSI_PORTAL_GROUP_FE,
            ISCSI_INITIATOR_GROUP) {
            Ok(_) => {
                Ok(Self { bdev_uuid_str: bdev_name.to_string() })
            },
            Err(e) => return Err(IscsiError::StartIscsi{ dev: bdev_name.to_string(), err: e.to_string() }),
        }
    }

    /// Stop and release iscsi device.
    pub async fn destroy(self) {
        info!("Destroying iscsi frontend target");
        match unshare(&self.bdev_uuid_str).await {
            Ok(_) => (),
            Err(e) =>  error!("Failed to destroy iscsi frontend target {}", e),
        }
    }

    /// Get device path actually means bdev_uuid in this case
    pub fn get_path(&self) -> String {
        return self.bdev_uuid_str.clone();
    }
}

impl fmt::Debug for IscsiTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{:?}", self.get_path(), self.bdev_uuid_str)
    }
}

impl fmt::Display for IscsiTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_path())
    }
}
