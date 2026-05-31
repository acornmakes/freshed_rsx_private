use std::borrow::Cow;
use std::sync::atomic::AtomicUsize;

pub struct NameGuard<'s>(Cow<'s, str>);

static NAME_GUARD_NUM: AtomicUsize = AtomicUsize::new(0);

impl<'s> NameGuard<'s> {
    pub fn new(name: &'s str) -> Self {
        Self(Cow::Borrowed(name))
    }

    pub fn unique() -> Self {
        let num = NAME_GUARD_NUM.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Self(Cow::Owned(format!("ng_{num}")))
    }

    pub fn get(&self) -> &str {
        self.0.as_ref()
    }
}
