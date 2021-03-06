use std::hash::Hash;
use std::hash::Hasher;
use std::option;
use std::default::Default;
use std::fmt;

pub struct SingularField<T>(Option<T>);

/// Like `Option<Box<T>>`, but keeps the actual element on `clear`.
pub struct SingularPtrField<T>(Option<Box<T>>);

impl<T> SingularField<T> {
    /// Construct this object from given value.
    #[inline]
    pub fn some(value: T) -> SingularField<T> {
        SingularField(Some(value))
    }

    /// True iff this object contains data.
    #[inline]
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// True iff this object contains no data.
    #[inline]
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    /// Convert this object into `Option`.
    #[inline]
    pub fn into_option(self) -> Option<T> {
        self.0
    }

    /// View data as `Option`.
    #[inline]
    pub fn as_ref<'a>(&'a self) -> Option<&'a T> {
        self.0.as_ref()
    }

    /// View data as mutable `Option`.
    #[inline]
    pub fn as_mut<'a>(&'a mut self) -> Option<&'a mut T> {
        self.0.as_mut()
    }

    /// Unwrap data as reference.
    #[inline]
    pub fn unwrap_ref<'a>(&'a self) -> &'a T {
        self.as_ref().unwrap()
    }

    /// Unwrap data as mutable reference.
    #[inline]
    pub fn unwrap_mut_ref<'a>(&'a mut self) -> &'a mut T {
        self.as_mut().unwrap()
    }

    /// Unwrap data, panic if not set.
    #[inline]
    pub fn unwrap(self) -> T {
        self.0.unwrap()
    }

    /// Unwrap data or return given default value.
    #[inline]
    pub fn unwrap_or(self, def: T) -> T {
        self.0.unwrap_or(def)
    }

    /// Unwrap data or return given default value.
    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F : FnOnce() -> T,
    {
        self.0.unwrap_or_else(f)
    }

    /// Apply a function to contained element and store result in new `SingularPtrField`.
    #[inline]
    pub fn map<U, F>(self, f: F) -> SingularPtrField<U>
    where
        F : FnOnce(T) -> U,
    {
        SingularPtrField::from_option(self.into_option().map(f))
    }

    /// View as iterator over references.
    #[inline]
    pub fn iter<'a>(&'a self) -> option::IntoIter<&'a T> {
        self.as_ref().into_iter()
    }

    /// View as iterator over mutable references.
    #[inline]
    pub fn mut_iter<'a>(&'a mut self) -> option::IntoIter<&'a mut T> {
        self.as_mut().into_iter()
    }

    /// Clear this object.
    /// Note, contained object destructor is not called, so allocated memory could be reused.
    #[inline]
    pub fn clear(&mut self) {
        self.0 = None;
    }
}

impl<T> SingularField<T> {
    /// Construct a `SingularField` with no data.
    #[inline]
    pub fn none() -> SingularField<T> {
        SingularField(None)
    }

    /// Construct `SingularField` from `Option`.
    #[inline]
    pub fn from_option(option: Option<T>) -> SingularField<T> {
        SingularField(option)
    }

    /// Return data as option, clear this object.
    #[inline]
    pub fn take(&mut self) -> Option<T> {
        self.0.take()
    }
}

impl<T: Default> SingularField<T> {
    /// Get contained data, consume self. Return default value for type if this is empty.
    #[inline]
    pub fn unwrap_or_default(self) -> T {
        self.0.unwrap_or_default()
    }

    /// Initialize this object with default value.
    /// This operation can be more efficient then construction of clear element,
    /// because it may reuse previously contained object.
    #[inline]
    pub fn set_default<'a>(&'a mut self) -> &'a mut T {
        self.0 = Some(Default::default());
        self.0.as_mut().unwrap()
    }
}

impl<T> SingularPtrField<T> {
    /// Construct `SingularPtrField` from given object.
    #[inline]
    pub fn some(value: T) -> SingularPtrField<T> {
        SingularPtrField(Some(Box::new(value)))
    }

    /// Construct an empty `SingularPtrField`.
    #[inline]
    pub fn none() -> SingularPtrField<T> {
        SingularPtrField(None)
    }

    /// Construct `SingularPtrField` from optional.
    #[inline]
    pub fn from_option(option: Option<T>) -> SingularPtrField<T> {
        SingularPtrField(option.map(Box::new))
    }

    /// True iff this object contains data.
    #[inline]
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// True iff this object contains no data.
    #[inline]
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    /// Convert into `Option<T>`.
    #[inline]
    pub fn into_option(self) -> Option<T> {
        self.0.map(|b| *b)
    }

    /// View data as reference option.
    #[inline]
    pub fn as_ref<'a>(&'a self) -> Option<&'a T> {
        self.0.as_ref().map(Box::as_ref)
    }

    /// View data as mutable reference option.
    #[inline]
    pub fn as_mut<'a>(&'a mut self) -> Option<&'a mut T> {
        self.0.as_mut().map(Box::as_mut)
    }

    /// Get data as reference.
    /// Panics if empty.
    #[inline]
    pub fn get_ref<'a>(&'a self) -> &'a T {
        self.as_ref().unwrap()
    }

    /// Get data as mutable reference.
    /// Panics if empty.
    #[inline]
    pub fn get_mut_ref<'a>(&'a mut self) -> &'a mut T {
        self.as_mut().unwrap()
    }

    /// Take the data.
    /// Panics if empty
    #[inline]
    pub fn unwrap(self) -> T {
        *self.0.unwrap()
    }

    /// Take the data or return supplied default element if empty.
    #[inline]
    pub fn unwrap_or(self, def: T) -> T {
        self.0.map(|b| *b).unwrap_or(def)
    }

    /// Take the data or return supplied default element if empty.
    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F : FnOnce() -> T,
    {
        self.0.map(|b| *b).unwrap_or_else(f)
    }

    /// Apply given function to contained data to construct another `SingularPtrField`.
    /// Returns empty `SingularPtrField` if this object is empty.
    #[inline]
    pub fn map<U, F>(self, f: F) -> SingularPtrField<U>
    where
        F : FnOnce(T) -> U,
    {
        SingularPtrField(self.0.map(|b| Box::new(f(*b))))
    }

    /// View data as iterator.
    #[inline]
    pub fn iter<'a>(&'a self) -> option::IntoIter<&'a T> {
        self.as_ref().into_iter()
    }

    /// View data as mutable iterator.
    #[inline]
    pub fn mut_iter<'a>(&'a mut self) -> option::IntoIter<&'a mut T> {
        self.as_mut().into_iter()
    }

    /// Take data as option, leaving this object empty.
    #[inline]
    pub fn take(&mut self) -> Option<T> {
        self.0.take().map(|b| *b)
    }
}

impl<T : Default> SingularPtrField<T> {
    /// Get contained data, consume self. Return default value for type if this is empty.
    #[inline]
    pub fn unwrap_or_default(self) -> T {
        self.0.map(|b| *b).unwrap_or_default()
    }

    /// Initialize this object with default value.
    /// This operation can be more efficient then construction of clear element,
    /// because it may reuse previously contained object.
    #[inline]
    pub fn set_default<'a>(&'a mut self) -> &'a mut T {
        self.0 = Some(Default::default());
        self.as_mut().unwrap()
    }
}

impl<T : Default> Default for SingularField<T> {
    #[inline]
    fn default() -> SingularField<T> {
        SingularField::none()
    }
}

impl<T> Default for SingularPtrField<T> {
    #[inline]
    fn default() -> SingularPtrField<T> {
        SingularPtrField::none()
    }
}

impl<T: Clone> Clone for SingularField<T> {
    #[inline]
    fn clone(&self) -> SingularField<T> {
        SingularField(self.0.clone())
    }
}

impl<T : Clone> Clone for SingularPtrField<T> {
    #[inline]
    fn clone(&self) -> SingularPtrField<T> {
        SingularPtrField(self.0.clone())
    }
}

impl<T : fmt::Debug> fmt::Debug for SingularField<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_some() {
            write!(f, "Some({:?})", *self.as_ref().unwrap())
        } else {
            write!(f, "None")
        }
    }
}

impl<T : fmt::Debug> fmt::Debug for SingularPtrField<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_some() {
            write!(f, "Some({:?})", *self.as_ref().unwrap())
        } else {
            write!(f, "None")
        }
    }
}

impl<T : PartialEq> PartialEq for SingularField<T> {
    #[inline]
    fn eq(&self, other: &SingularField<T>) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T : Eq> Eq for SingularField<T> {}

impl<T : PartialEq> PartialEq for SingularPtrField<T> {
    #[inline]
    fn eq(&self, other: &SingularPtrField<T>) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T : Eq> Eq for SingularPtrField<T> {}


impl<T : Hash> Hash for SingularField<T> {
    fn hash<H : Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<T : Hash> Hash for SingularPtrField<T> {
    fn hash<H : Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<'a, T> IntoIterator for &'a SingularField<T> {
    type Item = &'a T;
    type IntoIter = option::IntoIter<&'a T>;

    fn into_iter(self) -> option::IntoIter<&'a T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a SingularPtrField<T> {
    type Item = &'a T;
    type IntoIter = option::IntoIter<&'a T>;

    fn into_iter(self) -> option::IntoIter<&'a T> {
        self.iter()
    }
}
