pub mod schemas;
pub mod models;

mod theinfinitytimes_lib {
    pub trait PreSave {
        fn pre_save(&self) {
            panic!("Abstract method");
        }
    }
    pub trait PreSaveMut{
        fn pre_save(&mut self){ panic!("Abstract method")}
    }
}
