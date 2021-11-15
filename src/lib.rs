use std::path::PathBuf;

/// The hash-name part of a store path
/// ie: xmxgxig6zxrixicc7905ssgb4yc3lysa-bash-interactive-4.4-p23
#[derive(PartialEq, Eq, Debug)]
pub struct NarInfoId(String);

/// The hash-name part of a derivation's store path.
/// ie: a6xizp18g0sch9z7493p3irq632kzlym-bash-interactive-4.4-p23.drv
#[derive(PartialEq, Eq, Debug)]
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

#[derive(PartialEq, Eq, Debug)]
enum NarInfoDatum<'a> {
    Compression(&'a str),
    Deriver(DerivationId),
    FileHash(&'a str),
    FileSize(u64),
    NarHash(&'a str),
    NarSize(u64),
    References(Vec<NarInfoId>),
    Sig(&'a str),
    StorePath(PathBuf),
    Url(&'a str),
}

#[derive(PartialEq, Eq, Debug)]
enum ParseErr<'a> {
    LineCorruptNoColon(&'a str),
    LineUnknownKey(&'a str),
    InvalidU64(&'a str, std::num::ParseIntError),
    UnexpectedSpace(&'a str, usize),
}

type ParseResult = Result<NarInfo, ()>;

impl NarInfo {
    fn parse_str_no_spaces<'a>(key: &'a str, remainder: &'a str) -> Result<&'a str, ParseErr<'a>> {
        if let Some(position) = remainder.find(' ') {
            return Err(ParseErr::UnexpectedSpace(key, position));
        }

        Ok(remainder)
    }

    fn parse_u64<'a>(key: &'a str, remainder: &'a str) -> Result<u64, ParseErr<'a>> {
        remainder
            .parse::<u64>()
            .map_err(|e| ParseErr::InvalidU64(key, e))
    }

    fn parse_line(line: &str) -> Result<NarInfoDatum, ParseErr> {
        let (key, remainder): (&str, &str) = line
            .split_once(":")
            .ok_or(ParseErr::LineCorruptNoColon(line))?;

        let remainder = remainder.trim();

        match key {
            "Compression" => Ok(NarInfoDatum::Compression(Self::parse_str_no_spaces(
                key, remainder,
            )?)),
            "FileSize" => Ok(NarInfoDatum::FileSize(Self::parse_u64(key, remainder)?)),
            "NarSize" => Ok(NarInfoDatum::NarSize(Self::parse_u64(key, remainder)?)),
            unknown_key => Err(ParseErr::LineUnknownKey(unknown_key)),
        }
    }

    pub fn parse_string(nar: String) -> ParseResult {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_badly_formatted() {
        assert_eq!(
            NarInfo::parse_line("hello goodbye"),
            Err(ParseErr::LineCorruptNoColon("hello goodbye"))
        );
    }

    #[test]
    fn parse_u64_invalid_digit() {
        let ret = NarInfo::parse_u64("FooSize", "abc123");

        if let Err(ParseErr::InvalidU64("FooSize", err)) = ret {
            assert_eq!(err.kind(), &std::num::IntErrorKind::InvalidDigit);
        } else {
            panic!("Bad failure parsing: {:?}", ret);
        }
    }

    #[test]
    fn parse_str_unexpected_space_err() {
        assert_eq!(
            NarInfo::parse_str_no_spaces("FooStr", "foo bar baz"),
            Err(ParseErr::UnexpectedSpace("FooStr", 3))
        );
    }

    #[test]
    fn parse_str_unexpected_space() {
        assert_eq!(
            NarInfo::parse_str_no_spaces("FooStr", "foobarbaz"),
            Ok("foobarbaz")
        );
    }

    #[test]
    fn parse_line_narsize_invalid_digit() {
        let ret = NarInfo::parse_line("NarSize: abc123");

        if let Err(ParseErr::InvalidU64("NarSize", err)) = ret {
            assert_eq!(err.kind(), &std::num::IntErrorKind::InvalidDigit);
        } else {
            panic!("Bad failure parsing: {:?}", ret);
        }
    }

    #[test]
    fn parse_line_narsize() {
        assert_eq!(
            NarInfo::parse_line("NarSize: 234987234"),
            Ok(NarInfoDatum::NarSize(234987234))
        );
    }

    #[test]
    fn parse_line_filesize() {
        assert_eq!(
            NarInfo::parse_line("FileSize: 987987"),
            Ok(NarInfoDatum::FileSize(987987))
        );
    }

    #[test]
    fn parse_line_compression() {
        assert_eq!(
            NarInfo::parse_line("Compression: xz"),
            Ok(NarInfoDatum::Compression("xz"))
        );
    }
}
