use crate::{io::IO, meta::Meta, config::Config, hash::Hash};

pub struct FakeIO {
    pub f_is_materialized: bool,
    pub f_meta_exists: bool,
    pub f_hash: Hash,
    pub f_meta: Meta,
    pub f_config_exists: bool,
    pub f_config: Config
}
impl IO for FakeIO {
    fn materialize(&mut self, from: &std::path::Path) -> Result<crate::hash::Hash, crate::error::TRIError> {
        Ok(self.f_hash)
    }

    fn materialize_magick(&mut self, from: &crate::hash::Hash, cmd: &crate::magick::MagickCommand) -> Result<crate::hash::Hash, crate::error::TRIError>  {
        Ok(self.f_hash)
    }

    fn is_materialized(&mut self, hash: &crate::hash::Hash) -> bool {
        self.f_is_materialized
    }

    fn expose(&mut self, hash: &crate::hash::Hash, ext: Option<String>) -> Result<(), crate::error::TRIError> {
        Ok(())
    }

    fn meta_exists(&mut self) -> bool { self.f_meta_exists }

    fn meta_write(&mut self, meta: &crate::meta::Meta) -> Result<(), crate::error::TRIError> {
        Ok(())
    }

    fn meta_read(&mut self) -> Result<crate::meta::Meta, crate::error::TRIError> { Ok(self.f_meta.clone()) }

    fn config_exists(&mut self) -> bool { self.f_config_exists }

    fn config_write(&mut self, config: &crate::config::Config) -> Result<(), crate::error::TRIError> {
        Ok(())
    }

    fn config_read(&mut self) -> Result<crate::config::Config, crate::error::TRIError> {
        Ok(self.f_config.clone())
    }

    fn list_materialized(&mut self) -> Vec<crate::hash::Hash> {
        vec![self.f_hash]
    }

    fn watch_meta<TWatch>(&mut self, watch: TWatch) -> Result<notify::INotifyWatcher, crate::error::TRIError> where TWatch : FnMut(notify::Result<notify::Event>) + Send + 'static {
        todo!()
    }
}