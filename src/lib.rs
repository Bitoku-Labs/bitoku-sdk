/*
  Copyright 2023 Bitoku Labs

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use bitoku_sdk_agent_native::instruction::{register_client, send_request, Request};
use solana_program::{account_info::AccountInfo, program::invoke};
pub type Timestamp = i64;
pub type ClientId = u8;

pub struct CpiAccounts<'info> {
    pub fee_payer: AccountInfo<'info>,
    pub bookkeeper: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub request: AccountInfo<'info>,
    pub sys_var_program: AccountInfo<'info>,
    pub bitoku_agent_program_id: AccountInfo<'info>,
}

pub fn reg_client(accounts: &CpiAccounts) -> u8 {
    let ins = register_client(
        *accounts.fee_payer.key,
        *accounts.bookkeeper.key,
        *accounts.request.key,
        *accounts.system_program.key,
        *accounts.sys_var_program.key,
        *accounts.bitoku_agent_program_id.key,
    )
    .unwrap();

    let account_infos = [
        accounts.fee_payer.clone(),
        accounts.bookkeeper.clone(),
        accounts.request.clone(),
        accounts.system_program.clone(),
        accounts.sys_var_program.clone(),
    ];

    invoke(&ins, &account_infos).unwrap();

    let data = accounts.request.try_borrow_data().unwrap();
    let client_id = data[0];
    client_id
}

pub fn create_file(accounts: &CpiAccounts, file_name: String, data: [u8; 512], client_id: u8) {
    let mut arr: [u8; 128] = [0; 128];
    let bytes = file_name.as_bytes();
    arr[..bytes.len()].copy_from_slice(bytes);

    let req = Request::CreateFile {
        name: arr,
        data: data,
    };

    let ins = send_request(
        *accounts.fee_payer.key,
        *accounts.request.key,
        *accounts.bitoku_agent_program_id.key,
        client_id,
        req,
    )
    .unwrap();

    let account_infos = [
        accounts.fee_payer.clone(),
        accounts.bookkeeper.clone(),
        accounts.request.clone(),
        accounts.system_program.clone(),
        accounts.sys_var_program.clone(),
    ];

    invoke(&ins, &account_infos).unwrap();
}

pub fn write_file(
    accounts: &CpiAccounts,
    file_name: &String,
    data: [u8; 512],
    file_id: u8,
    client_id: u8,
) {
    let mut arr: [u8; 128] = [0; 128];
    let bytes = file_name.as_bytes();
    arr[..bytes.len()].copy_from_slice(bytes);

    let req = Request::WriteFile {
        name: arr,
        file_id: file_id,
        data: data,
    };

    let ins = send_request(
        *accounts.fee_payer.key,
        *accounts.request.key,
        *accounts.bitoku_agent_program_id.key,
        client_id,
        req,
    )
    .unwrap();

    let account_infos = [
        accounts.fee_payer.clone(),
        accounts.bookkeeper.clone(),
        accounts.request.clone(),
        accounts.system_program.clone(),
        accounts.sys_var_program.clone(),
    ];

    invoke(&ins, &account_infos).unwrap();

    let mut arr: [u8; 128] = [0; 128];
    let bytes = file_name.as_bytes();
    arr[..bytes.len()].copy_from_slice(bytes);

}

