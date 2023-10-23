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
use std::io::Cursor;

#[test]
pub fn parse_get_request_from_string() -> serde_json::Result<()> {
    let request1 = credentialhelper::GetCredentialsRequest::deserialize(Cursor::new(
        "{\"uri\": \"grpcs://example.com\"}",
    ))?;
    assert_eq!("grpcs://example.com", request1.uri());

    let request2 = credentialhelper::GetCredentialsRequest::deserialize(Cursor::new(
        "{\"uri\": \"grpcs://example.org\"}",
    ))?;
    assert_eq!("grpcs://example.org", request2.uri());

    return Ok(());
}

#[test]
pub fn parse_get_request_from_string_with_extra_fields() -> serde_json::Result<()> {
    let request = credentialhelper::GetCredentialsRequest::deserialize(Cursor::new(
        "{\"foo\": 1, \"uri\": \"grpcs://example.com\", \"bar\": 2}",
    ))?;
    assert_eq!("grpcs://example.com", request.uri());

    return Ok(());
}

#[test]
pub fn parse_get_response_from_string() -> serde_json::Result<()> {
    let response = credentialhelper::GetCredentialsResponse::deserialize(Cursor::new(
        "{\"headers\": {\"header1\": [\"value1\"],\"header2\": [\"value1\", \"value2\"],\"header3\": [\"value1\", \"value2\", \"value3\"]}}",
    ))?;
    assert_eq!(
        HashMap::from([
            ("header1".to_string(), ["value1".to_string()].to_vec()),
            (
                "header2".to_string(),
                ["value1".to_string(), "value2".to_string()].to_vec()
            ),
            (
                "header3".to_string(),
                [
                    "value1".to_string(),
                    "value2".to_string(),
                    "value3".to_string()
                ]
                .to_vec()
            )
        ]),
        response.headers()
    );

    return Ok(());
}

#[test]
pub fn parse_get_response_from_string_with_extra_fields() -> serde_json::Result<()> {
    let response = credentialhelper::GetCredentialsResponse::deserialize(Cursor::new(
        "{\"foo\": 1, \"headers\": {\"foo\": [\"1\"], \"bar\": [\"2\"]}, \"bar\": 12}",
    ))?;
    assert_eq!(
        HashMap::from([
            ("foo".to_string(), ["1".to_string()].to_vec()),
            ("bar".to_string(), ["2".to_string()].to_vec())
        ]),
        response.headers()
    );

    return Ok(());
}
