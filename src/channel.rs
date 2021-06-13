
pub enum Channel {
    CONTROL,
    SENSOR,
    VIDEO,
    INPUT,
    AUDIO,
    AUDIO1,
    AUDIO2,
    MIC,
    BLUETOOTH,
    MUSIC_PLAYBACK,
    NAVIGATION_DIRECTION,
    NOTIFICATION,
    PHONE_STATUS
}

impl Channel {
    pub fn asInt(&self) -> u8 {
        match &self  {
            Channel::CONTROL => 0,
            Channel::SENSOR => 1,
            Channel::VIDEO => 2,
            Channel::INPUT => 3,
            Channel::AUDIO => 6,
            Channel::AUDIO1 => 4,
            Channel::AUDIO2 => 5,
            Channel::MIC => 7,
            Channel::BLUETOOTH => 8,
            Channel::MUSIC_PLAYBACK => 9,
            Channel::NAVIGATION_DIRECTION => 10,
            Channel::NOTIFICATION => 11,
            Channel::PHONE_STATUS => 12
        }
    }
}