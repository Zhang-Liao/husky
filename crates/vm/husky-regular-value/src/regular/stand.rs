use super::*;

pub trait __RegularStand: std::fmt::Debug + RefUnwindSafe + UnwindSafe + 'static {
    type __Snapshot: __RegularSnapshot<__Incubator = Self::__Incubator>;
    type __Incubator: __RegularIncubator<__Stand = Self>;

    unsafe fn clone_into_snapshot(&self) -> Self::__Snapshot;
}

#[derive(Debug)]
pub struct __RegularValueStandRefMut<T>(pub(super) *mut T)
where
    T: __RegularStand;

impl<T> __RegularStand for __RegularValueStandRefMut<T>
where
    T: __RegularStand,
{
    type __Snapshot = __RegularSnapshotValueRefMut<T>;

    type __Incubator = __RegularValueIncubatorRefMut<T>;

    unsafe fn clone_into_snapshot(&self) -> Self::__Snapshot {
        __RegularSnapshotValueRefMut(Box::new((*self.0).clone_into_snapshot()))
    }
}

pub trait __RegularStandDyn: std::fmt::Debug + std::any::Any + RefUnwindSafe + UnwindSafe {
    unsafe fn clone_into_arc_snapshot(&self) -> Arc<dyn __RegularSnapshotDyn>;
}

impl<T> __RegularStandDyn for T
where
    T: __RegularStand,
{
    unsafe fn clone_into_arc_snapshot(&self) -> Arc<dyn __RegularSnapshotDyn> {
        Arc::new(self.clone_into_snapshot())
    }
}