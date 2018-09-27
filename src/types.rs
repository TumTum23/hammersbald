//
// Copyright 2018 Tamas Blummer
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//!
//! # Types used in db files
//! Offset an unsigned 48 bit integer used as file offset
//! U24 an unsigned 24 bit integer for data element sizes

use error::BCDBError;
use page::PAGE_SIZE;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
pub struct Offset(u64);

impl Offset {

    pub fn new (value: u64) ->Result<Offset, BCDBError> {
        if value > 1 << 47 {
            return Err(BCDBError::InvalidOffset);
        }
        Ok(Offset(value))
    }

    pub fn from_slice (slice: &[u8]) -> Result<Offset, BCDBError> {
        if slice.len() != 6 {
            return Err(BCDBError::InvalidOffset);
        }
        let mut size = 0u64;
        for i in 0 .. 6 {
            size <<= 8;
            size += slice[i] as u64;
        }
        Ok(Offset(size))
    }

    pub fn as_u64 (&self) -> u64 {
        return self.0;
    }

    pub fn serialize (&self, into: &mut [u8]) {
        use std::mem::transmute;

        let bytes: [u8; 8] = unsafe { transmute(self.0.to_be()) };
        into.copy_from_slice(&bytes[2 .. 8]);
    }

    pub fn this_page(&self) -> Offset {
        Offset::new((self.0/ PAGE_SIZE as u64)* PAGE_SIZE as u64).unwrap()
    }

    pub fn next_page(&self) -> Result<Offset, BCDBError> {
        Offset::new((self.0/ PAGE_SIZE as u64 + 1)* PAGE_SIZE as u64)
    }

    pub fn page_number(&self) -> u64 {
        self.0/PAGE_SIZE as u64
    }

    pub fn in_page_pos(&self) -> usize {
        (self.0 - (self.0/ PAGE_SIZE as u64)* PAGE_SIZE as u64) as usize
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
pub struct U24 (usize);

impl U24 {

    pub fn new (value: usize) ->Result<U24, BCDBError> {
        if value > 1 << 23 {
            return Err(BCDBError::InvalidOffset);
        }
        Ok(U24(value))
    }

    pub fn from_slice (slice: &[u8]) -> Result<U24, BCDBError> {
        if slice.len() != 3 {
            return Err(BCDBError::InvalidOffset);
        }
        let mut size = 0usize;
        for i in 0 .. 3 {
            size <<= 8;
            size += slice[i] as usize;
        }
        Ok(U24(size))
    }

    pub fn as_usize (&self) -> usize {
        return self.0;
    }

    pub fn serialize (&self, into: &mut [u8]) {
        use std::mem::transmute;

        let bytes: [u8; 8] = unsafe { transmute(self.0.to_be()) };
        into.copy_from_slice(&bytes[5 .. 8]);
    }
}

