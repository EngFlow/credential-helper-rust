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
use std::collections::HashMap;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::AcqRel;

struct CountingCache {
    count: AtomicU32,
}

impl CredentialHelper for CountingCache {
    #[allow(unused_variables)]
    fn get_credentials(
        &self,
        request: GetCredentialsRequest,
        additional_parameters: Vec<String>,
    ) -> Result<GetCredentialsResponse, String> {
        return Ok(GetCredentialsResponse::new(HashMap::from([
            ("uri".to_string(), vec![request.uri()]),
            (
                "count".to_string(),
                vec![self.count.fetch_add(1, AcqRel).to_string()],
            ),
        ])));
    }
}

#[test]
pub fn caches_credentials() {
    let helper = credentialhelpercache::Cache::<CountingCache>::builder(CountingCache {
        count: AtomicU32::new(0),
    })
    .build()
    .unwrap();

    let response1 = helper
        .get_credentials(
            GetCredentialsRequest::new("grpcs://example.com/foo"),
            vec![],
        )
        .unwrap();
    assert_eq!(
        "grpcs://example.com/foo",
        response1
            .headers()
            .get("uri")
            .unwrap()
            .get(0)
            .unwrap()
            .to_owned()
    );
    assert_eq!(
        "0",
        response1
            .headers()
            .get("count")
            .unwrap()
            .get(0)
            .unwrap()
            .to_owned()
    );

    let response2 = helper
        .get_credentials(
            GetCredentialsRequest::new("grpcs://example.com/foo"),
            vec![],
        )
        .unwrap();
    assert_eq!(
        "grpcs://example.com/foo",
        response2
            .headers()
            .get("uri")
            .unwrap()
            .get(0)
            .unwrap()
            .to_owned()
    );
    assert_eq!(
        "0",
        response2
            .headers()
            .get("count")
            .unwrap()
            .get(0)
            .unwrap()
            .to_owned()
    );

    let response3 = helper
        .get_credentials(
            GetCredentialsRequest::new("grpcs://example.com/bar"),
            vec![],
        )
        .unwrap();
    assert_eq!(
        "grpcs://example.com/bar",
        response3
            .headers()
            .get("uri")
            .unwrap()
            .get(0)
            .unwrap()
            .to_owned()
    );
    assert_eq!(
        "1",
        response3
            .headers()
            .get("count")
            .unwrap()
            .get(0)
            .unwrap()
            .to_owned()
    );
}
