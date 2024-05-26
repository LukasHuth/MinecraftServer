use std::fmt::Display;

/// An enum of all banner pattern variants
///
/// # Source
///
/// - [here](https://minecraft.fandom.com/wiki/Chunk_format#Block_entity_format)
///   - banners
///     - pattern
#[allow(missing_docs)]
#[derive(PartialEq, Eq, Debug)]
pub enum BannerPatternVariant {
    Base,
    BottomStripe,
    TopStripe,
    LeftStripe,
    RightStripe,
    CenterStripe,
    MiddleStripe,
    DownRightStripe,
    DownLeftStripe,
    SmallStripes,
    DiagonalCross,
    SquareCross,
    LeftOfDiagonal,
    RightOfUpsizeDownDiagonal,
    LeftOFUpsizeDownDiagonal,
    RightOfDiagonal,
    VerticalHalf,
    VerticalHalfRight,
    HorizontalHalf,
    HorizontalHalfBottom,
    BottomLeftCorner,
    BottomRightCorner,
    TopLeftCorner,
    TopRightCorner,
    BottomTriangle,
    TopTriangle,
    BottomTriangleSawtooth,
    TopTriangleSawtooth,
    MiddleCircle,
    Border,
    CurlyBorder,
    Brick,
    Gradient,
    GradientUpsideDown,
    Creeper,
    Skull,
    Flower,
    Mojang,
    Globe,
    Piglin,
}
impl Display for BannerPatternVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base => f.write_str("b"),
            Self::BottomStripe => f.write_str("bs"),
            Self::TopStripe => f.write_str("ts"),
            Self::LeftStripe => f.write_str("ls"),
            Self::RightStripe => f.write_str("rs"),
            Self::CenterStripe => f.write_str("cs"),
            Self::MiddleStripe => f.write_str("ms"),
            Self::DownRightStripe => f.write_str("drs"),
            Self::DownLeftStripe => f.write_str("dls"),
            Self::SmallStripes => f.write_str("ss"),
            Self::DiagonalCross => f.write_str("cr"),
            Self::SquareCross => f.write_str("sc"),
            Self::LeftOfDiagonal => f.write_str("ld"),
            Self::RightOfUpsizeDownDiagonal => f.write_str("rud"),
            Self::LeftOFUpsizeDownDiagonal => f.write_str("lud"),
            Self::RightOfDiagonal => f.write_str("rd"),
            Self::VerticalHalf => f.write_str("vh"),
            Self::VerticalHalfRight => f.write_str("vhr"),
            Self::HorizontalHalf => f.write_str("hh"),
            Self::HorizontalHalfBottom => f.write_str("hhb"),
            Self::BottomLeftCorner => f.write_str("bl"),
            Self::BottomRightCorner => f.write_str("br"),
            Self::TopLeftCorner => f.write_str("tl"),
            Self::TopRightCorner => f.write_str("tr"),
            Self::BottomTriangle => f.write_str("bt"),
            Self::TopTriangle => f.write_str("tt"),
            Self::BottomTriangleSawtooth => f.write_str("bts"),
            Self::TopTriangleSawtooth => f.write_str("tts"),
            Self::MiddleCircle => f.write_str("mc"),
            Self::Border => f.write_str("bo"),
            Self::CurlyBorder => f.write_str("cbo"),
            Self::Brick => f.write_str("bri"),
            Self::Gradient => f.write_str("gra"),
            Self::GradientUpsideDown => f.write_str("gru"),
            Self::Creeper => f.write_str("cre"),
            Self::Skull => f.write_str("sku"),
            Self::Flower => f.write_str("flo"),
            Self::Mojang => f.write_str("moj"),
            Self::Globe => f.write_str("glb"),
            Self::Piglin => f.write_str("pig"),
        }
    }
}
/// A Struct managing one pattern of a banner
#[derive(PartialEq, Debug)]
pub struct BannerPattern {
    /// The color of the pattern
    pub color: minecraft_assets::color::Color,
    /// The pattern variant
    pub variant: BannerPatternVariant,
}
