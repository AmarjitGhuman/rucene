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

use std::ops::Deref;

use core::analysis::TokenStream;
use core::doc::SORTED_NUMERIC_DOC_VALUES_FIELD_TYPE;
use core::doc::{Field, FieldType};
use core::index::Fieldable;
use core::util::{numeric::Numeric, VariantValue};

use error::Result;

pub struct SortedNumericDocValuesField {
    field: Field,
}

impl SortedNumericDocValuesField {
    pub fn new(name: &str, value: i64) -> SortedNumericDocValuesField {
        SortedNumericDocValuesField {
            field: Field::new(
                String::from(name),
                SORTED_NUMERIC_DOC_VALUES_FIELD_TYPE,
                Some(VariantValue::Long(value)),
                None,
            ),
        }
    }

    pub fn numeric_value(&self) -> i64 {
        match self.field.fields_data().unwrap() {
            VariantValue::Long(v) => *v,
            _ => unreachable!(),
        }
    }
}

impl Fieldable for SortedNumericDocValuesField {
    fn name(&self) -> &str {
        self.field.name()
    }

    fn field_type(&self) -> &FieldType {
        self.field.field_type()
    }

    fn boost(&self) -> f32 {
        self.field.boost()
    }

    fn fields_data(&self) -> Option<&VariantValue> {
        self.field.fields_data()
    }

    fn token_stream(&mut self) -> Result<Box<dyn TokenStream>> {
        self.field.token_stream()
    }

    fn binary_value(&self) -> Option<&[u8]> {
        None
    }

    fn string_value(&self) -> Option<&str> {
        None
    }

    fn numeric_value(&self) -> Option<Numeric> {
        self.field.numeric_value()
    }
}

impl Deref for SortedNumericDocValuesField {
    type Target = Field;

    fn deref(&self) -> &Field {
        &self.field
    }
}
