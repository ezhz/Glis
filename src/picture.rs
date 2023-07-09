
use std::{io, fmt, path::*};
use image::
{
    GenericImageView,
    DynamicImage::*,
};

// ------------------------------------------------------------

#[derive(Debug)]
pub enum PictureError
{
    IO(std::io::Error),
    ImageError(image::error::ImageError),
    UnsupportedChannelCount(u8),
    UnsupportedImageFormat,
    UnsupportedPixelFormat
}

impl std::error::Error for PictureError {}

impl fmt::Display for PictureError
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self 
        {
            Self::IO(error) => write!(formatter, "{}", error),
            Self::ImageError(error) => write!(formatter, "{error}"),
            Self::UnsupportedChannelCount(count)
                => write!(formatter, "Unsupported channel count {count}"),
            Self::UnsupportedImageFormat
                => write!(formatter, "Unsupported image format"),
            Self::UnsupportedPixelFormat
                => write!(formatter, "Unsupported pixel format")
        }
    }
}

pub type PictureResult<T> = std::result::Result<T, PictureError>;

// ------------------------------------------------------------

#[derive(Clone, Copy)]
pub enum ChannelCount
{
    One,
    Two,
    Three,
    Four
}

impl TryFrom<u8> for ChannelCount
{
    type Error = PictureError;
    fn try_from(number: u8) -> PictureResult<Self>
    {
        match number
        {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            _ => Err(PictureError::UnsupportedChannelCount(number))
        }
    }
}

// ------------------------------------------------------------

#[derive(Clone)]
pub enum PixelData
{
    EightBit(Vec<u8>),
    SixteenBit(Vec<u16>),
    ThirtyTwoBit(Vec<f32>)
}

// ------------------------------------------------------------

pub struct PictureData
{
    pub pixel_data: PixelData,
    pub resolution: [u32; 2],
    pub channels: ChannelCount
}

impl TryFrom<image::DynamicImage> for PictureData
{
    type Error = PictureError;
    fn try_from(dynamic_image: image::DynamicImage) -> PictureResult<Self>
    {
        use PixelData::*;
        let resolution = dynamic_image.dimensions();
        let resolution = [resolution.0, resolution.1];
        let color_type = dynamic_image.color();
        let channel_count = color_type.channel_count();
        let channel_count = ChannelCount::try_from(channel_count)?;
        let data = match dynamic_image
        {
            ImageLuma8(buffer) => EightBit(buffer.into_raw()),
            ImageLumaA8(buffer) => EightBit(buffer.into_raw()),
            ImageRgb8(buffer) => EightBit(buffer.into_raw()),
            ImageRgba8(buffer) => EightBit(buffer.into_raw()),
            ImageLuma16(buffer) => SixteenBit(buffer.into_raw()),
            ImageLumaA16(buffer) => SixteenBit(buffer.into_raw()),
            ImageRgb16(buffer) => SixteenBit(buffer.into_raw()),
            ImageRgba16(buffer) => SixteenBit(buffer.into_raw()),
            ImageRgb32F(buffer) => ThirtyTwoBit(buffer.into_raw()),
            ImageRgba32F(buffer) => ThirtyTwoBit(buffer.into_raw()),
            _ => return Err(PictureError::UnsupportedPixelFormat)
        };
        let this = Self
        {
            pixel_data: data, 
            resolution, 
            channels: channel_count
        };
        Ok(this)
    }
}

// ------------------------------------------------------------

pub struct Picture(image::DynamicImage);

impl Picture
{
    pub fn open(path: impl AsRef<Path>) -> PictureResult<Self>
    {
        image::io::Reader::open(path).map_err(PictureError::IO)
            .and_then(Self::try_from)
    }

    pub fn flipv(&mut self) -> ()
    {
        self.0 = self.0.flipv()
    }
}

impl<R> TryFrom<image::io::Reader<R>> for Picture
where
    R: io::Read + io::BufRead + io::Seek + 'static
{
    type Error = PictureError;
    fn try_from(reader: image::io::Reader<R>) -> PictureResult<Self>
    {
        reader.format()
            .ok_or(PictureError::UnsupportedImageFormat)?;
        reader.decode().map(Self)
            .map_err(PictureError::ImageError)
    }
}

impl TryFrom<Picture> for PictureData
{
    type Error = PictureError;
    fn try_from(picture: Picture) -> PictureResult<Self>
    {
        picture.0.try_into()
    }
}

