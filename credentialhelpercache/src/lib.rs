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

use credentialhelper::CredentialHelper;
use credentialhelper::GetCredentialsRequest;
use credentialhelper::GetCredentialsResponse;
use std::sync::Arc;

pub struct Cache<CredentialHelperT: CredentialHelper> {
    delegate: CredentialHelperT,
    cache: moka::sync::Cache<GetCredentialsRequest, GetCredentialsResponse>,
}

impl<CredentialHelperT: CredentialHelper> Cache<CredentialHelperT> {
    pub fn builder<T: CredentialHelper>(credential_helper: T) -> CacheBuilder<T> {
        return CacheBuilder {
            delegate: credential_helper,
            cache: moka::sync::Cache::builder(),
        };
    }
}

impl<CredentialHelperT: CredentialHelper> CredentialHelper for Cache<CredentialHelperT> {
    #[allow(unknown_lints)]
    #[allow(arc_into_inner)]
    fn get_credentials(
        &self,
        request: GetCredentialsRequest,
        additional_parameters: Vec<String>,
    ) -> Result<GetCredentialsResponse, String> {
        let response = self.cache.try_get_with(request.clone(), || {
            return self
                .delegate
                .get_credentials(request, additional_parameters);
        });
        match response {
            Ok(v) => return Ok(v),
            Err(msg) => match Arc::into_inner(msg) {
                Some(s) => return Err(s),
                None => {
                    return Err("Encountered unknown error while fetching credentials".to_string())
                }
            },
        };
    }
}

pub struct CacheBuilder<CredentialHelperT: CredentialHelper> {
    delegate: CredentialHelperT,
    cache: moka::sync::CacheBuilder<
        GetCredentialsRequest,
        GetCredentialsResponse,
        moka::sync::Cache<GetCredentialsRequest, GetCredentialsResponse>,
    >,
}

impl<CredentialHelperT: CredentialHelper> CacheBuilder<CredentialHelperT> {
    pub fn build(self) -> Result<Cache<CredentialHelperT>, ()> {
        return Ok(Cache {
            delegate: self.delegate,
            cache: self.cache.build(),
        });
    }
}
