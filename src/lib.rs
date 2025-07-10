use std::{
    ffi::c_void,
    fmt::{self, Display},
    marker::PhantomData,
};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct GixDLList<T> {
    gdll: *mut GixDLL,
    phantom: PhantomData<T>,
}

pub struct GixNodeWrapper<T> {
    ptr: *mut GixNode,
    phantom: PhantomData<T>,
}

impl<T> GixDLList<T> {
    pub fn new() -> Option<GixDLList<T>> {
        let a = unsafe {
            let res = __internal_gix_dll_new(size_of::<T>());

            if res.is_null() {
                None
            } else {
                Some(Self {
                    gdll: res,
                    phantom: PhantomData,
                })
            }
        };

        a
    }

    pub fn size(&self) -> usize {
        unsafe { (*self.gdll).size }
    }

    pub fn append(&self, val: &T) -> Option<GixNodeWrapper<T>> {
        let res = unsafe {
            let node = gix_dll_append(self.gdll, val as *const T as *const c_void);

            if node.is_null() {
                None
            } else {
                Some(GixNodeWrapper {
                    ptr: node,
                    phantom: PhantomData,
                })
            }
        };

        res
    }

    pub fn prepend(&self, val: &T) -> Option<GixNodeWrapper<T>> {
        let res = unsafe {
            let node = gix_dll_prepend(self.gdll, val as *const T as *const c_void);

            if node.is_null() {
                None
            } else {
                Some(GixNodeWrapper {
                    ptr: node,
                    phantom: PhantomData,
                })
            }
        };
        res
    }

    pub fn insert_after(&self, node: &GixNodeWrapper<T>, val: &T) -> Option<GixNodeWrapper<T>> {
        let res = unsafe {
            let node = gix_dll_insert_after(self.gdll, node.ptr, val as *const T as *const c_void);
            if node.is_null() {
                None
            } else {
                Some(GixNodeWrapper {
                    ptr: node,
                    phantom: PhantomData,
                })
            }
        };
        res
    }
    pub fn insert_before(&self, node: &GixNodeWrapper<T>, val: &T) -> Option<GixNodeWrapper<T>> {
        let res = unsafe {
            let node = gix_dll_insert_before(self.gdll, node.ptr, val as *const T as *const c_void);
            if node.is_null() {
                None
            } else {
                Some(GixNodeWrapper {
                    ptr: node,
                    phantom: PhantomData,
                })
            }
        };
        res
    }

    pub fn remove(&self, node: GixNodeWrapper<T>) {
        unsafe {
            gix_dll_remove(self.gdll, node.ptr);
        }
    }

    pub fn get_value_at(&self, index: usize) -> Option<&T> {
        let res = unsafe {
            let a = gix_dll_get_value_at(self.gdll, index) as *const T;
            if a.is_null() { None } else { Some(&*a) }
        };
        res
    }

    pub fn remove_at(&self, index: usize) {
        unsafe {
            gix_dll_remove_at(self.gdll, index);
        }
    }

    pub fn set_data_at(&self, index: usize, val: &T) {
        unsafe {
            gix_dll_set_value_at(self.gdll, index, val as *const T as *const c_void);
        }
    }

    pub fn as_ref(&self) -> &GixDLL {
        unsafe { &*self.gdll }
    }

    pub fn as_mut_ref(&self) -> &mut GixDLL {
        unsafe { &mut *self.gdll }
    }

    pub fn as_ptr(&self) -> *const GixDLL {
        self.gdll
    }

    pub fn as_mut_ptr(&self) -> *mut GixDLL {
        self.gdll
    }
}

impl<T> Drop for GixDLList<T> {
    fn drop(&mut self) {
        unsafe {
            gix_dll_destroy(self.gdll);
        }
    }
}

impl<T: std::fmt::Debug> Display for GixDLList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let mut current = (*self.gdll).head;
            write!(f, "[")?;

            let mut first = true;
            while !current.is_null() {
                let data_ptr = gix_node_get_value(current) as *const T;
                if !data_ptr.is_null() {
                    if !first {
                        write!(f, ", ")?;
                    }
                    let val = &*data_ptr;
                    write!(f, "{:?}", val)?;
                    first = false;
                }

                current = gix_node_next(current);
            }

            write!(f, "]")
        }
    }
}

impl<T> GixNodeWrapper<T> {
    pub unsafe fn from_raw(ptr: *mut GixNode) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self {
                ptr: ptr,
                phantom: PhantomData,
            })
        }
    }

    pub fn set_data(&self, val: &T) {
        unsafe {
            gix_node_set_value(self.ptr, val as *const T as *const c_void);
        }
    }

    pub fn get_data<'a>(&self) -> Option<&'a T> {
        unsafe {
            let val_ptr = gix_node_get_value(self.ptr) as *const T;
            if val_ptr.is_null() {
                None
            } else {
                Some(&*val_ptr)
            }
        }
    }

    pub fn next(&self) -> Option<Self> {
        unsafe { Self::from_raw(gix_node_next(self.ptr)) }
    }

    pub fn prev(&self) -> Option<Self> {
        unsafe { Self::from_raw(gix_node_prev(self.ptr)) }
    }

    pub fn as_ref(&self) -> &GixNode {
        unsafe { &*self.ptr }
    }

    pub fn as_mut_ref(&self) -> &mut GixNode {
        unsafe { &mut *self.ptr }
    }

    pub fn as_ptr(&self) -> *const GixNode {
        self.ptr
    }

    pub fn as_mut_ptr(&self) -> *mut GixNode {
        self.ptr
    }
}
