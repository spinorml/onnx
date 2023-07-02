/*
 * Licensed to SpinorML under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * SpinorML licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::{fs::File, path::PathBuf};

use onnxrust::onnx_proto3::ModelProto;
use protobuf::{CodedInputStream, Message};

#[test]
fn test_read_single_relu() {
    let mut resource_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    resource_path.push("onnx/onnx/examples/resources/single_relu.onnx");

    let mut file = File::open(resource_path).unwrap();
    let mut stream = CodedInputStream::new(&mut file);
    let model = ModelProto::parse_from(&mut stream).unwrap();

    assert_eq!("backend-test", model.producer_name);
}
