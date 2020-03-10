use std::{marker::PhantomData, ptr::NonNull};

use spine_sys::{spSkeletonJson, spSkeletonJson_create, spSkeletonJson_dispose};

use super::atlas::Atlas;

#[repr(transparent)]
pub struct SkeletonJson<'atlas>(
    pub(crate) NonNull<spSkeletonJson>,
    pub(crate) PhantomData<&'atlas ()>,
);

impl<'a> SkeletonJson<'a> {
    pub fn from_atlas(atlas: &'a Atlas) -> Self {
        let pointer = unsafe { spSkeletonJson_create(atlas.0.as_ptr()) };
        let pointer = NonNull::new(pointer).unwrap();
        Self(pointer, PhantomData)
    }

    pub fn set_scale(&mut self, scale: f32) -> &mut Self {
        unsafe { self.0.as_mut().scale = scale }
        self
    }
}

impl<'a> Drop for SkeletonJson<'a> {
    fn drop(&mut self) {
        unsafe { spSkeletonJson_dispose(self.0.as_ptr()) }
    }
}