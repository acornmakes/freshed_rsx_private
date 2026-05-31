use std::sync::atomic::AtomicUsize;

pub enum NameGuard<'s> {
    Str(&'s str),
    Dyn(String),
}

static NAME_GUARD_NUM: AtomicUsize = AtomicUsize::new(0);

impl<'s> NameGuard<'s> {
    pub fn new(name: &'s str) -> Self {
        Self::Str(name)
    }

    pub fn unique() -> Self {
        let num = NAME_GUARD_NUM.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Self::Dyn(format!("ng_{num}"))
    }

    pub fn get(&'s self) -> &'s str {
        match self {
            Self::Str(s) => s,
            Self::Dyn(s) => s,
        }
    }
}
