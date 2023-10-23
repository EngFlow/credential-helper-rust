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

use std::io::Cursor;
use std::process::Command;
use std::process::Stdio;

use crate::credential_helper::CredentialHelper;
use crate::protocol::GetCredentialsRequest;
use crate::protocol::GetCredentialsResponse;

pub struct Client {
    credential_helper_path: String,
}

impl Client {
    pub fn new<S: Into<String>>(credential_helper_path: S) -> Result<Client, ()> {
        return Ok(Client {
            credential_helper_path: credential_helper_path.into(),
        });
    }
}

impl CredentialHelper for Client {
    fn get_credentials(
        &self,
        request: GetCredentialsRequest,
        additional_parameters: Vec<String>,
    ) -> Result<GetCredentialsResponse, String> {
        match Command::new(self.credential_helper_path.as_str())
            .arg("get")
            .args(additional_parameters)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(mut child) => {
                // Intentionally ignore errors from writing to child's `stdin`.
                _ = request.serialize(child.stdin.as_mut().unwrap());

                match child.wait_with_output() {
                    Ok(output) => {
                        match GetCredentialsResponse::deserialize(Cursor::new(output.stdout)) {
                            Ok(response) => {
                                return Ok(response);
                            }
                            Err(_) => {
                                return Err(
                                    "Could not read response from credential helper".to_string()
                                );
                            }
                        }
                    }
                    Err(_) => {
                        return Err("Could not read response from credential helper".to_string());
                    }
                };
            }
            Err(_) => {
                return Err("Could not start credential helper".to_string());
            }
        };
    }
}
