use std::path::PathBuf;

use crate::{Commitment, RegisteredPoStProof, PrivateSectorPathInfo};

// A byte serialized representation of a vanilla proof.
pub type VanillaProofBytes = Vec<u8>;

/// The minimal information required about a replica, in order to be able to generate
/// a PoSt over it.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrivateReplicaInfo {
    /// The version of this replica.
    pub(crate) registered_proof: RegisteredPoStProof,
    /// The replica commitment.
    pub(crate) comm_r: Commitment,
    /// Contains sector-specific (e.g. merkle trees) assets
    pub(crate) cache_dir: PathBuf,
    /// Contains the replica.
    pub(crate) replica_path: PathBuf,
    /// Cache file is stored in OSS
    pub(crate) cache_in_oss: bool,
    /// Replica is stored in OSS
    pub(crate) replica_in_oss: bool,
    /// OSS info of cache
    pub(crate) cache_sector_path_info: PrivateSectorPathInfo,
    /// OSS info of replica
    pub(crate) replica_sector_path_info: PrivateSectorPathInfo,
}

impl PrivateReplicaInfo {
    pub fn new(
        registered_proof: RegisteredPoStProof,
        comm_r: Commitment,
        cache_dir: PathBuf,
        replica_path: PathBuf,
    ) -> Self {
        PrivateReplicaInfo {
            registered_proof,
            comm_r,
            cache_dir,
            replica_path,
            cache_in_oss: false,
            replica_in_oss: false,
            cache_sector_path_info: Default::default(),
            replica_sector_path_info: Default::default(),
        }
    }

    pub fn new_with_oss_config(
        registered_proof: RegisteredPoStProof,
        replica_path: PathBuf,
        replica_in_oss: bool,
        replica_sector_path_info: PrivateSectorPathInfo,
        comm_r: Commitment,
        cache_dir: PathBuf,
        cache_in_oss: bool,
        cache_sector_path_info: PrivateSectorPathInfo
    )-> Self {
        PrivateReplicaInfo {
            registered_proof,
            comm_r,
            cache_dir,
            replica_path,
            cache_in_oss,
            replica_in_oss,
            cache_sector_path_info,
            replica_sector_path_info,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PublicReplicaInfo {
    /// The version of this replica.
    pub(crate) registered_proof: RegisteredPoStProof,
    /// The replica commitment.
    pub(crate) comm_r: Commitment,
}

impl PublicReplicaInfo {
    pub fn new(registered_proof: RegisteredPoStProof, comm_r: Commitment) -> Self {
        PublicReplicaInfo {
            registered_proof,
            comm_r,
        }
    }
}
