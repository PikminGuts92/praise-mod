pub trait AudioMeta {
    fn get_channel_count(&self) -> u8;
    fn get_sample_rate(&self) -> u32;
}