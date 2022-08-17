use crate::data::{artist::Artist, release_group::ReleaseGroup};

pub trait Id {
    fn id(&self) -> String;
}

impl Id for Artist {
    fn id(&self) -> String {
        self.id()
    }
}

impl Id for ReleaseGroup {
    fn id(&self) -> String {
        self.id()
    }
}

pub trait ReadableForm {
    fn readable_from(&self) -> String;
}

impl ReadableForm for Artist {
    fn readable_from(&self) -> String {
        self.name()
    }
}

impl ReadableForm for ReleaseGroup {
    fn readable_from(&self) -> String {
        format!("{} - {}", self.release_type(), self.title())
    }
}
