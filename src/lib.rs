mod theinfinitytimes_lib {
    pub trait PreSave {
        fn pre_save(&self) {
            panic!("Abstract method");
        }
    }
}