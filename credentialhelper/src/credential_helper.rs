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

use crate::GetCredentialsRequest;
use crate::GetCredentialsResponse;

/// Represents a Credential Helper.
///
/// Note: we intentionally require all implementations of `CredentialHelper` to be thread- and async safe.
pub trait CredentialHelper: Send + Sync {
    #[allow(unused_variables)]
    fn get_credentials(
        &self,
        request: GetCredentialsRequest,
        additional_parameters: Vec<String>,
    ) -> Result<GetCredentialsResponse, String> {
        return Err("Credential Helper does not support command 'get'".to_string());
    }
}
