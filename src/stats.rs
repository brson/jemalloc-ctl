//! Global allocator statistics.
//!
//! jemalloc tracks a wide variety of statistics. Many of them are cached, and only refreshed when
//! the jemalloc "epoch" is advanced. See the [`Epoch`] type for more information.
//!
//! [`Epoch`]: ../struct.Epoch.html

use std::io;
use std::os::raw::c_char;

use {get, get_mib, name_to_mib};

const ALLOCATED: *const c_char = b"stats.allocated\0" as *const _ as *const _;

/// Returns the total number of bytes allocated by the application.
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`epoch`]
/// function for more information.
///
/// This corresponds to `stats.allocated` in jemalloc's API.
///
/// # Examples
///
/// ```
/// let a = jemalloc_ctl::stats::allocated().unwrap();
/// let _buf = vec![0; 1024 * 1024];
/// jemalloc_ctl::epoch().unwrap();
/// let b = jemalloc_ctl::stats::allocated().unwrap();
/// assert!(a < b);
/// ```
///
/// [`epoch`]: ../fn.epoch().html
pub fn allocated() -> io::Result<usize> {
    unsafe { get(ALLOCATED) }
}

/// A type providing access to the total number of bytes allocated by the application.
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`Epoch`]
/// type for more information.
///
/// This corresponds to `stats.allocated` in jemalloc's API.
///
/// # Examples
///
/// ```rust
/// use jemalloc_ctl::Epoch;
/// use jemalloc_ctl::stats::Allocated;
///
/// let epoch = Epoch::new().unwrap();
/// let allocated = Allocated::new().unwrap();
///
/// let a = allocated.get().unwrap();
/// let _buf = vec![0; 1024 * 1024];
/// epoch.advance().unwrap();
/// let b = allocated.get().unwrap();
/// assert!(a < b);
/// ```
///
/// [`Epoch`]: ../struct.Epoch.html
#[derive(Copy, Clone)]
pub struct Allocated([usize; 2]);

impl Allocated {
    /// Returns a new `Allocated`.
    pub fn new() -> io::Result<Allocated> {
        let mut mib = [0; 2];
        unsafe {
            name_to_mib(ALLOCATED, &mut mib)?;
        }
        Ok(Allocated(mib))
    }

    /// Returns the total number of bytes allocated by the application.
    pub fn get(&self) -> io::Result<usize> {
        unsafe { get_mib(&self.0) }
    }
}

const ACTIVE: *const c_char = b"stats.active\0" as *const _ as *const _;

/// Returns the total number of bytes in active pages allocated by the application.
///
/// This is a multiple of the page size, and is greater than or equal to the value returned by
/// [`allocated`].
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`epoch`]
/// type for more information.
///
/// This corresponds to `stats.active` in jemalloc's API.
///
/// # Examples
///
/// ```
/// let a = jemalloc_ctl::stats::active().unwrap();
/// let _buf = vec![0; 1024 * 1024];
/// jemalloc_ctl::epoch().unwrap();
/// let b = jemalloc_ctl::stats::active().unwrap();
/// assert!(a < b);
/// ```
///
/// [`epoch`]: ../fn.epoch().html
/// [`allocated`]: fn.allocated.hml
pub fn active() -> io::Result<usize> {
    unsafe { get(ACTIVE) }
}

/// A type providing access to the total number of bytes in active pages allocated by the
/// application.
///
/// This is a multiple of the page size, and greater than or equal to the value returned by
/// [`Allocated`].
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`Epoch`]
/// type for more information.
///
/// This corresponds to `stats.active` in jemalloc's API.
///
/// # Examples
///
/// ```rust
/// use jemalloc_ctl::Epoch;
/// use jemalloc_ctl::stats::Active;
///
/// let epoch = Epoch::new().unwrap();
/// let active = Active::new().unwrap();
///
/// let a = active.get().unwrap();
/// let _buf = vec![0; 1024 * 1024];
/// epoch.advance().unwrap();
/// let b = active.get().unwrap();
/// assert!(a < b);
/// ```
///
/// [`Epoch`]: ../struct.Epoch.html
/// [`Allocated`]: struct.Allocated.html
#[derive(Copy, Clone)]
pub struct Active([usize; 2]);

impl Active {
    /// Returns a new `Allocated`.
    pub fn new() -> io::Result<Active> {
        let mut mib = [0; 2];
        unsafe {
            name_to_mib(ACTIVE, &mut mib)?;
        }
        Ok(Active(mib))
    }

    /// Returns the total number of bytes in active pages allocated by the application.
    pub fn get(&self) -> io::Result<usize> {
        unsafe { get_mib(&self.0) }
    }
}

const METADATA: *const c_char = b"stats.metadata\0" as *const _ as *const _;

/// Returns the total number of bytes dedicated to jemalloc metadata.
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`epoch`]
/// function for more information.
///
/// This corresponds to `stats.metadata` in jemalloc's API.
///
/// # Examples
///
/// ```
/// jemalloc_ctl::epoch().unwrap();
/// println!("{} bytes of jemalloc metadata", jemalloc_ctl::stats::metadata().unwrap());
/// ```
///
/// [`epoch`]: ../fn.epoch.html
pub fn metadata() -> io::Result<usize> {
    unsafe { get(METADATA) }
}

/// A type providing access to the total number of bytes dedicated to jemalloc metadata.
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`Epoch`]
/// type for more information.
///
/// This corresponds to `stats.metadata` in jemalloc's API.
///
/// # Examples
///
/// ```rust
/// use jemalloc_ctl::Epoch;
/// use jemalloc_ctl::stats::Metadata;
///
/// let epoch = Epoch::new().unwrap();
/// let metadata = Metadata::new().unwrap();
///
/// epoch.advance().unwrap();
/// let size = metadata.get().unwrap();
/// println!("{} bytes of jemalloc metadata", size);
/// ```
///
/// [`Epoch`]: ../struct.Epoch.html
#[derive(Copy, Clone)]
pub struct Metadata([usize; 2]);

impl Metadata {
    /// Returns a new `Metadata`.
    pub fn new() -> io::Result<Metadata> {
        let mut mib = [0; 2];
        unsafe {
            name_to_mib(METADATA, &mut mib)?;
        }
        Ok(Metadata(mib))
    }

    /// Returns the total number of bytes dedicated to jemalloc metadata.
    pub fn get(&self) -> io::Result<usize> {
        unsafe { get_mib(&self.0) }
    }
}

const RESIDENT: *const c_char = b"stats.resident\0" as *const _ as *const _;

/// Returns the total number of bytes in physically resident data pages mapped by the allocator.
///
/// This consists of all pages dedicated to allocator metadata, pages backing active allocations,
/// and unused dirty pages. It may overestimate the true value because pages may not actually be
/// physically resident if they correspond to demand-zeroed virtual memory that has not yet been
/// touched. This is a multiple of the page size, and is larger than the value returned by
/// [`active`].
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`epoch`]
/// function for more information.
///
/// This corresponds to `stats.resident` in jemalloc's API.
///
/// # Examples
///
/// ```
/// jemalloc_ctl::epoch().unwrap();
/// println!("{} bytes of total resident data", jemalloc_ctl::stats::resident().unwrap());
/// ```
///
/// [`epoch`]: ../fn.epoch.html
/// [`active`]: fn.active.html
pub fn resident() -> io::Result<usize> {
    unsafe { get(RESIDENT) }
}

/// A type providing access to the total number of bytes in physically resident data pages mapped
/// by the allocator.
///
/// This consists of all pages dedicated to allocator metadata, pages backing active allocations,
/// and unused dirty pages. It may overestimate the true value because pages may not actually be
/// physically resident if they correspond to demand-zeroed virtual memory that has not yet been
/// touched. This is a multiple of the page size, and is larger than the value returned by
/// [`Active`].
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`Epoch`]
/// type for more information.
///
/// This corresponds to `stats.resident` in jemalloc's API.
///
/// # Examples
///
/// ```rust
/// use jemalloc_ctl::Epoch;
/// use jemalloc_ctl::stats::Resident;
///
/// let epoch = Epoch::new().unwrap();
/// let resident = Resident::new().unwrap();
///
/// epoch.advance().unwrap();
/// let size = resident.get().unwrap();
/// println!("{} bytes of total resident data", size);
/// ```
///
/// [`Epoch`]: ../struct.Epoch.html
/// [`Active`]: struct.Active.html
#[derive(Copy, Clone)]
pub struct Resident([usize; 2]);

impl Resident {
    /// Returns a new `Resident`.
    pub fn new() -> io::Result<Resident> {
        let mut mib = [0; 2];
        unsafe {
            name_to_mib(RESIDENT, &mut mib)?;
        }
        Ok(Resident(mib))
    }

    /// Returns the total number of bytes in physically resident data pages mapped by the allocator.
    pub fn get(&self) -> io::Result<usize> {
        unsafe { get_mib(&self.0) }
    }
}

const MAPPED: *const c_char = b"stats.mapped\0" as *const _ as *const _;

/// Returns the total number of bytes in active extents mapped by the allocator.
///
/// This does not include inactive extents, even those taht contain unused dirty pages, so there
/// is no strict ordering between this and the value returned by [`resident`]. This is a
/// multiple of the page size, and is larger than the value returned by [`active`].
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`epoch`]
/// type for more information.
///
/// This corresponds to `stats.mapped` in jemalloc's API.
///
/// # Examples
///
/// ```
/// jemalloc_ctl::epoch().unwrap();
/// println!("{} bytes of total mapped data", jemalloc_ctl::stats::mapped().unwrap());
/// ```
///
/// [`epoch`]: ../fn.epoch.html
/// [`resident`]: fn.resident.html
/// [`active`]: fn.active.html
pub fn mapped() -> io::Result<usize> {
    unsafe { get(MAPPED) }
}

/// A type providing access to the total number of bytes in active extents mapped by the allocator.
///
/// This does not include inactive extents, even those that contain unused dirty pages, so there
/// is no strict ordering between this and the value returned by [`Resident`]. This is a
/// multiple of the page size, and is larger than the value returned by [`Active`].
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`Epoch`]
/// type for more information.
///
/// This corresponds to `stats.mapped` in jemalloc's API.
///
/// # Examples
///
/// ```rust
/// use jemalloc_ctl::Epoch;
/// use jemalloc_ctl::stats::Mapped;
///
/// let epoch = Epoch::new().unwrap();
/// let mapped = Mapped::new().unwrap();
///
/// epoch.advance().unwrap();
/// let size = mapped.get().unwrap();
/// println!("{} bytes of total mapped data", size);
/// ```
///
/// [`Epoch`]: ../struct.Epoch.html
/// [`Resident`]: struct.Resident.html
/// [`Active`]: struct.Active.html
#[derive(Copy, Clone)]
pub struct Mapped([usize; 2]);

impl Mapped {
    /// Returns a new `Mapped`.
    pub fn new() -> io::Result<Mapped> {
        let mut mib = [0; 2];
        unsafe {
            name_to_mib(MAPPED, &mut mib)?;
        }
        Ok(Mapped(mib))
    }

    /// Returns the total number of bytes in active extents mapped by the allocator.
    pub fn get(&self) -> io::Result<usize> {
        unsafe { get_mib(&self.0) }
    }
}
