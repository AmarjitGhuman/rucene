// Copyright 2019 Zhizhesihai (Beijing) Technology Limited.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Copy, Clone)]
pub(crate) struct ReaderSlice {
    pub start: i32,
    pub length: i32,
    pub reader_index: usize,
}

impl ReaderSlice {
    pub fn new(start: i32, length: i32, reader_index: usize) -> ReaderSlice {
        ReaderSlice {
            start,
            length,
            reader_index,
        }
    }
}
