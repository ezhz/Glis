
use super::picture;

// ------------------------------------------------------------

impl From<picture::ChannelCount> for ogl::ChannelCount
{
    fn from(count: picture::ChannelCount) -> Self
    {
        match count
        {
            picture::ChannelCount::One => Self::One,
            picture::ChannelCount::Two => Self::Two,
            picture::ChannelCount::Three => Self::Three,
            picture::ChannelCount::Four => Self::Four    
        }
    }
}

