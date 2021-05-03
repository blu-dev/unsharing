use std::sync::atomic::*;
use skyline::nn;

use crate::{offsets::offset_to_addr, utils::NnMutexGuard};
use crate::vector::CppVector;
use smash_arc::{LoadedArc, LoadedSearchSection};

pub static mut LOADED_TABLES_OFFSET: usize = 0;

#[repr(C)]
pub struct PathEntry {
    pub loaded_index: u32,
    pub is_loaded: u32
}

impl PathEntry {
    pub fn is_loaded(&self) -> bool {
        self.is_loaded != 0
    }
}

#[repr(u8)]
pub enum LoadedFileState {
    Unloaded = 0,
    Unused = 1,
    Unk = 2,
    Loaded = 3
}

#[repr(C)]
pub struct LoadedEntry {
    pub file_data: *mut u8,
    pub ref_count: AtomicU32,
    pub is_used: bool,
    pub state: LoadedFileState,
    pub flags2: bool,
    pub flags: u8,
    pub version: u32,
    pub unk: u8
}

#[repr(C)]
pub struct LoadedDirectory {
    pub load_data_index: u32,
    pub dir_count: u32,
    pub _x8: u64,
    pub child_files_indexes: CppVector<u32>,
    pub child_directories: CppVector<*mut LoadedDirectory>,
    pub redirection_dir: *mut LoadedDirectory
}

#[repr(C)]
pub struct LoadedData {
    pub arc: &'static mut LoadedArc,
    pub search: &'static mut LoadedSearchSection
}

#[repr(C)]
pub struct LoadedTables {
    pub mutex: *mut nn::os::MutexType,
    pub paths: *mut PathEntry,
    pub loaded: *mut LoadedEntry,
    pub paths_len: u32,
    pub loaded_len: u32,
    pub paths_count: u32,
    pub loaded_count: u32,
    pub path_list: CppVector<u32>,
    pub loaded_directories: *mut LoadedDirectory,
    pub loaded_directories_len: u32,
    pub unk: u32,
    pub unk2: CppVector<u32>,
    pub unk3: u64,
    pub addr: *const (),
    pub loaded_data: &'static mut LoadedData,
    pub version: u32
}

#[allow(dead_code)]
impl LoadedTables {
    pub fn lock(&mut self) -> NnMutexGuard<Self> {
        NnMutexGuard::new(self.mutex, self)
    }

    pub fn lock_instance() -> NnMutexGuard<'static, Self> {
        Self::get_instance().lock()
    }

    pub fn paths(&self) -> &[PathEntry] {
        unsafe {
            std::slice::from_raw_parts(self.paths, self.paths_len as usize)
        }
    }

    pub fn paths_mut(&mut self) -> &mut [PathEntry] {
        unsafe {
            std::slice::from_raw_parts_mut(self.paths, self.paths_len as usize)
        }
    }

    pub fn loaded(&self) -> &[LoadedEntry] {
        unsafe {
            std::slice::from_raw_parts(self.loaded, self.loaded_len as usize)
        }
    }

    pub fn loaded_mut(&mut self) -> &mut [LoadedEntry] {
        unsafe {
            std::slice::from_raw_parts_mut(self.loaded, self.loaded_len as usize)
        }
    }

    pub fn loaded_directories(&self) -> &[LoadedDirectory] {
        unsafe {
            std::slice::from_raw_parts(self.loaded_directories, self.loaded_directories_len as usize)
        }
    }

    pub fn loaded_directories_mut(&mut self) -> &mut [LoadedDirectory] {
        unsafe {
            std::slice::from_raw_parts_mut(self.loaded_directories, self.loaded_directories_len as usize)
        }
    }

    pub fn get_instance() -> &'static mut Self {
        unsafe {
            let instance_ptr = offset_to_addr(LOADED_TABLES_OFFSET) as *mut &'static mut Self;
            *instance_ptr
        }
    }

    pub fn get_arc() -> &'static LoadedArc {
        Self::get_instance().loaded_data.arc
    }

    pub fn get_arc_mut() -> &'static mut LoadedArc {
        Self::get_instance().loaded_data.arc
    }
}

use std::ops::{Index, IndexMut};

impl Index<PathEntry> for [LoadedEntry] {
    type Output = LoadedEntry;
    
    fn index(&self, index: PathEntry) -> &Self::Output {
        self.get(index.loaded_index as usize).unwrap()
    }
}

impl IndexMut<PathEntry> for [LoadedEntry] {
    fn index_mut(&mut self, index: PathEntry) -> &mut Self::Output {
        self.get_mut(index.loaded_index as usize).unwrap()
    }
}