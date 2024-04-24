// Copyright (C) 2022 Satoshi Konno All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub type Result<T> = std::result::Result<T, Error>;

pub struct ScoketError {}

impl ScoketError {
    pub fn new(msg: &str) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, msg)
    }
}

pub struct BindError {}

impl BindError {
    pub fn new() -> std::io::Error {
        ScoketError::new("socket is not bound")
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub msg: String,
}

impl Error {
    /// from_str creates a new Error with the specified string.
    pub fn from_str(str: &str) -> Error {
        Error {
            msg: str.to_string(),
        }
    }

    /// from_string creates a new Error with the specified string.
    pub fn from_string(str: &String) -> Error {
        Error { msg: str.clone() }
    }

    /// message returns the error message.
    pub fn message(&self) -> &str {
        &self.msg
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
