use std::path::PathBuf;

/// The hash-name part of a store path
/// ie: xmxgxig6zxrixicc7905ssgb4yc3lysa-bash-interactive-4.4-p23
pub struct NarInfoId(String);

/// The hash-name part of a derivation's store path.
/// ie: a6xizp18g0sch9z7493p3irq632kzlym-bash-interactive-4.4-p23.drv
pub struct DerivationId(String);

/// A parsed NarInfo file, which can be fetched from
/// https://cache.nixos.org/storepathhash.narinfo.
/// For example:
///     curl https://cache.nixos.org/xmxgxig6zxrixicc7905ssgb4yc3lysa.narinfo
pub struct NarInfo {
    /// The location on disk the NAR this narinfo points to. Example:
    /// /nix/store/xmxgxig6zxrixicc7905ssgb4yc3lysa-bash-interactive-4.4-p23
    pub storepath: PathBuf,

    /// A relative path one level up from this narinfo to the NAR. For example,
    /// if the narinfo is at https://cache.nixos.org/foo.narinfo and the url
    /// is `nar/bar.nar.xz`, the nar lives at https://cache.nixos.org/nar/bar.nar.xz.
    pub url: String,

    /// The compression algorithm used on the NAR.
    pub compression: String,

    /// The hash of the compressed NAR
    pub file_hash: String,

    /// The size of the compressed NAR
    pub file_size: u64,

    /// The hash of the decompressed NAR
    pub nar_hash: String,

    /// The size of the decompressed NAR
    pub nar_size: u64,

    /// Other NARs this NAR's store path depends on
    pub references: Vec<NarInfoId>,

    /// The name of the Derivation used to build this store path
    pub deriver: DerivationId,

    /// The signature which is against the contents of the narinfo minus the signature line.
    pub signature: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
