pub fn const_ptr<T, U>(val: &T) -> *const U {
    return (val as *const T) as *const U;
}

pub fn mut_ptr<T, U>(val: &mut T) -> *mut U {
    return (val as *mut T) as *mut U;
}
