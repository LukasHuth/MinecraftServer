/// Packet that answers, how it proceded after the `AddResoucePack` packet
pub struct ResoucePackResponse {
    /// id of the requested resource pack
    pub uuid: datatypes::UUID,
    /// result, how the request went
    pub result: datatypes::Enum<resource_pack_response_extra::Result, datatypes::VarInt>,
}
#[allow(missing_docs)]
pub mod resource_pack_response_extra {
    use datatypes::ImportantEnumTrait;

    /// The result of the Resource Pack Response
    pub enum Result {
        /// The client was able to download the resource pack
        SuccessfullyDownloaded,
        /// The client decliened to add the resource pack
        Declined,
        /// The client was not able to download the resource pack
        FailedToDownload,
        /// the Resource pack request was accepted
        Accepted,
        /// The resourcepack was already downloaded
        Downloaded,
        /// The client could not establish a connection to the download url
        InvalidUrl,
        /// The client was unable to reload its resource packs
        FailedToReload,
        /// The downloaded resource pack was discarded, because the hash was not correct
        Discarded,
    }
    impl ImportantEnumTrait for Result {
        fn new(data: u64) -> binary_utils::Result<Self> {
            match data {
                0 => Ok(Self::SuccessfullyDownloaded),
                1 => Ok(Self::Declined),
                2 => Ok(Self::FailedToDownload),
                3 => Ok(Self::Accepted),
                4 => Ok(Self::Downloaded),
                5 => Ok(Self::InvalidUrl),
                6 => Ok(Self::FailedToReload),
                7 => Ok(Self::Discarded),
                8..=u64::MAX => Err(binary_utils::Error::InvalidStructure),
            }
        }
    }
}
