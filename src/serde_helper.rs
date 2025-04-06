
// purpose here is to implement serde serializers for iced theme.
// iced theme however is an enum, using palette as a potential custom entry
// this in turn also uses color
// all in all, there is a lot to work around to get this to serialize
// see https://serde.rs/remote-derive.html
// TODO: the enum{Custom(Arc<Custom>)} makes all of this very tricky. therefore skip it for now
// for this to work at all, remember to add feature "rc" to serde in the Cargo.toml
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use iced::Theme;
use iced::theme::Custom;
/*
use iced::Color;
use iced::theme::palette;
use iced::theme::palette::{Pair, Primary, Secondary, Background, Palette, Extended, Success, Danger };

#[derive(Serialize, Deserialize)]
#[serde(remote = "Color")]
pub struct ColorDef{
    /// Red component, 0.0 - 1.0
    pub r: f32,
    /// Green component, 0.0 - 1.0
    pub g: f32,
    /// Blue component, 0.0 - 1.0
    pub b: f32,
    /// Transparency, 0.0 - 1.0
    pub a: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Palette")]
pub struct PaletteDef {
    /// The background [`Color`] of the [`Palette`].
    #[serde(with = "ColorDef")]
    pub background: Color,
    /// The text [`Color`] of the [`Palette`].
    #[serde(with = "ColorDef")]
    pub text: Color,
    /// The primary [`Color`] of the [`Palette`].
    #[serde(with = "ColorDef")]
    pub primary: Color,
    /// The success [`Color`] of the [`Palette`].
    #[serde(with = "ColorDef")]
    pub success: Color,
    /// The danger [`Color`] of the [`Palette`].
    #[serde(with = "ColorDef")]
    pub danger: Color,
}

/// A pair of background and text colors.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Pair")]
pub struct PairDef {
    /// The background color.
    #[serde(with = "ColorDef")]
    pub color: Color,

    /// The text color.
    ///
    /// It's guaranteed to be readable on top of the background [`color`].
    ///
    /// [`color`]: Self::color
    #[serde(with = "ColorDef")]
    pub text: Color,
}

/// A set of background colors.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Background")]
pub struct BackgroundDef {
    /// The base background color.
    #[serde(with = "PairDef")]
    pub base: Pair,
    /// A weaker version of the base background color.
    #[serde(with = "PairDef")]
    pub weak: Pair,
    /// A stronger version of the base background color.
    #[serde(with = "PairDef")]
    pub strong: Pair,
}

/// A set of primary colors.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Primary")]
pub struct PrimaryDef {
    /// The base primary color.
    #[serde(with = "PairDef")]
    pub base: Pair,
    /// A weaker version of the base primary color.
    #[serde(with = "PairDef")]
    pub weak: Pair,
    /// A stronger version of the base primary color.
    #[serde(with = "PairDef")]
    pub strong: Pair,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Secondary")]
pub struct SecondaryDef {
    /// The base secondary color.
    #[serde(with = "PairDef")]
    pub base: Pair,
    /// A weaker version of the base secondary color.
    #[serde(with = "PairDef")]
    pub weak: Pair,
    /// A stronger version of the base secondary color.
    #[serde(with = "PairDef")]
    pub strong: Pair,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Success")]
pub struct SuccessDef {
    /// The base success color.
    #[serde(with = "PairDef")]
    pub base: Pair,
    /// A weaker version of the base success color.
    #[serde(with = "PairDef")]
    pub weak: Pair,
    /// A stronger version of the base success color.
    #[serde(with = "PairDef")]
    pub strong: Pair,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Danger")]
pub struct DangerDef {
    /// The base danger color.
    #[serde(with = "PairDef")]
    pub base: Pair,
    /// A weaker version of the base danger color.
    #[serde(with = "PairDef")]
    pub weak: Pair,
    /// A stronger version of the base danger color.
    #[serde(with = "PairDef")]
    pub strong: Pair,
}

/// An extended set of colors generated from a [`Palette`].
#[derive(Serialize, Deserialize)]
#[serde(remote = "Extended")]
pub struct ExtendedDef {
    /// The set of background colors.
    #[serde(with = "BackgroundDef")]
    pub background: Background,
    /// The set of primary colors.
    #[serde(with = "PrimaryDef")]
    pub primary: Primary,
    /// The set of secondary colors.
    #[serde(with = "SecondaryDef")]
    pub secondary: Secondary,
    /// The set of success colors.
    #[serde(with = "SuccessDef")]
    pub success: Success,
    /// The set of danger colors.
    #[serde(with = "DangerDef")]
    pub danger: Danger,
    /// Whether the palette is dark or not.
    pub is_dark: bool,
}

/// A [`Theme`] with a customized [`Palette`].
#[derive(Serialize, Deserialize)]
#[serde(remote = "Custom")]
pub struct CustomDef {
    name: String,
    #[serde(with = "PaletteDef")]
    palette: Palette,
    #[serde(with = "ExtendedDef")]
    extended: palette::Extended,
}

impl From<CustomDef> for Custom {
    fn from(custom: CustomDef) -> Custom {
        Custom::new(custom.name, customm.palette);
    }
}*/


#[derive(Serialize, Deserialize)]
#[serde(remote = "Theme")]
pub enum ThemeDef{
    /// The built-in light variant.
    Light,
    /// The built-in dark variant.
    Dark,
    /// The built-in Dracula variant.
    Dracula,
    /// The built-in Nord variant.
    Nord,
    /// The built-in Solarized Light variant.
    SolarizedLight,
    /// The built-in Solarized Dark variant.
    SolarizedDark,
    /// The built-in Gruvbox Light variant.
    GruvboxLight,
    /// The built-in Gruvbox Dark variant.
    GruvboxDark,
    /// The built-in Catppuccin Latte variant.
    CatppuccinLatte,
    /// The built-in Catppuccin Frappé variant.
    CatppuccinFrappe,
    /// The built-in Catppuccin Macchiato variant.
    CatppuccinMacchiato,
    /// The built-in Catppuccin Mocha variant.
    CatppuccinMocha,
    /// The built-in Tokyo Night variant.
    TokyoNight,
    /// The built-in Tokyo Night Storm variant.
    TokyoNightStorm,
    /// The built-in Tokyo Night Light variant.
    TokyoNightLight,
    /// The built-in Kanagawa Wave variant.
    KanagawaWave,
    /// The built-in Kanagawa Dragon variant.
    KanagawaDragon,
    /// The built-in Kanagawa Lotus variant.
    KanagawaLotus,
    /// The built-in Moonfly variant.
    Moonfly,
    /// The built-in Nightfly variant.
    Nightfly,
    /// The built-in Oxocarbon variant.
    Oxocarbon,
    /// The built-in Ferra variant:
    Ferra,
    // A [`Theme`] that uses a [`Custom`] palette.
    // #[serde(with = "CustomDef")]
    // TODO: we have all the custom values defined for serde,
    // but how can we use it in the Arc in an Enum?
    #[serde(skip)]
    Custom(Arc<Custom>),
}
