//  Copyright 2022. The Tari Project
//
//  Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//  following conditions are met:
//
//  1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//  disclaimer.
//
//  2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//  following disclaimer in the documentation and/or other materials provided with the distribution.
//
//  3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//  products derived from this software without specific prior written permission.
//
//  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//  USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use tari_template_abi::{Decode, Encode};

use crate::models::{ContractAddress, PackageId};

pub type ComponentId = crate::Hash;

#[derive(Debug, Clone, Encode, Decode)]
pub struct ComponentInstance {
    pub component_id: ComponentId,
    pub contract_address: ContractAddress,
    pub package_id: PackageId,
    pub module_name: String,
    pub state: Vec<u8>,
}

impl ComponentInstance {
    pub fn new(component_id: ComponentId, component: Component) -> Self {
        Self {
            component_id,
            contract_address: component.contract_address,
            package_id: component.package_id,
            module_name: component.module_name,
            state: component.state,
        }
    }

    pub fn id(&self) -> ComponentId {
        self.component_id
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct Component {
    pub contract_address: ContractAddress,
    pub package_id: PackageId,
    pub module_name: String,
    pub state: Vec<u8>,
}
