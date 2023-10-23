// Copyright 2023 EngFlow, Inc. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::io::Read;
use std::io::Write;

/// GetCredentialsRequest represents the request for the `get` command of the Helper Protocol.
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetCredentialsRequest {
    uri: String,
}

impl GetCredentialsRequest {
    pub fn new<S: Into<String>>(uri: S) -> GetCredentialsRequest {
        return GetCredentialsRequest { uri: uri.into() };
    }

    pub fn deserialize<R: Read>(input: R) -> serde_json::Result<GetCredentialsRequest> {
        return serde_json::from_reader(input);
    }

    pub fn serialize<W: Write>(&self, output: W) -> serde_json::Result<()> {
        return serde_json::to_writer(output, self);
    }

    pub fn uri(&self) -> String {
        return self.uri.clone();
    }
}

/// GetCredentialsResponse represents the response for the `get` command of the Helper Protocol.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetCredentialsResponse {
    headers: HashMap<String, Vec<String>>,
}

impl GetCredentialsResponse {
    pub fn new(headers: HashMap<String, Vec<String>>) -> GetCredentialsResponse {
        return GetCredentialsResponse { headers };
    }

    pub fn deserialize<R: Read>(input: R) -> serde_json::Result<GetCredentialsResponse> {
        return serde_json::from_reader(input);
    }

    pub fn serialize<W: Write>(&self, output: W) -> serde_json::Result<()> {
        return serde_json::to_writer(output, self);
    }

    pub fn headers(&self) -> HashMap<String, Vec<String>> {
        return self.headers.clone();
    }
}
