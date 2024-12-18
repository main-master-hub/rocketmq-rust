/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::collections::HashMap;

use cheetah_string::CheetahString;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrokerMemberGroup {
    pub cluster: CheetahString,
    pub broker_name: CheetahString,
    pub broker_addrs: HashMap<u64 /* brokerId */, CheetahString /* broker address */>,
}

impl BrokerMemberGroup {
    pub fn new(cluster: CheetahString, broker_name: CheetahString) -> Self {
        Self {
            cluster,
            broker_name,
            broker_addrs: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBrokerMemberGroupResponseBody {
    pub broker_member_group: Option<BrokerMemberGroup>,
}
