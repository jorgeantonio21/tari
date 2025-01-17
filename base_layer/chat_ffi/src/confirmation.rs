// Copyright 2023, The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::{convert::TryFrom, ptr};

use libc::{c_int, c_uint};
use tari_contacts::contacts_service::types::Confirmation;

use crate::{
    error::{InterfaceError, LibChatError},
    types::{chat_byte_vector_create, ChatByteVector},
};

/// Get a pointer to a ChatByteVector representation of a message id
///
/// ## Arguments
/// `confirmation` - A pointer to the Confirmation
/// `error_out` - Pointer to an int which will be modified
///
/// ## Returns
/// `*mut ChatByteVector` - A ptr to a ChatByteVector
///
/// # Safety
/// The ```confirmation``` When done with the confirmation it should be destroyed
/// The ```ChatByteVector``` When done with the returned ChatByteVector it should be destroyed
#[no_mangle]
pub unsafe extern "C" fn read_confirmation_message_id(
    confirmation: *mut Confirmation,
    error_out: *mut c_int,
) -> *mut ChatByteVector {
    let mut error = 0;
    ptr::swap(error_out, &mut error as *mut c_int);

    if confirmation.is_null() {
        error = LibChatError::from(InterfaceError::NullError("client".to_string())).code;
        ptr::swap(error_out, &mut error as *mut c_int);
    }

    let c = &(*confirmation);
    let data_bytes = c.message_id.clone();
    let len = u32::try_from(data_bytes.len()).expect("Can't cast from usize");
    chat_byte_vector_create(data_bytes.as_ptr(), len as c_uint, error_out)
}

/// Get a c_uint timestamp for the confirmation
///
/// ## Arguments
/// `confirmation` - A pointer to the Confirmation
/// `error_out` - Pointer to an int which will be modified
///
/// ## Returns
/// `c_uint` - A uint representation of time. May return 0 if casting fails
///
/// # Safety
/// None
#[no_mangle]
pub unsafe extern "C" fn read_confirmation_timestamp(confirmation: *mut Confirmation, error_out: *mut c_int) -> c_uint {
    let mut error = 0;
    ptr::swap(error_out, &mut error as *mut c_int);

    if confirmation.is_null() {
        error = LibChatError::from(InterfaceError::NullError("client".to_string())).code;
        ptr::swap(error_out, &mut error as *mut c_int);
    }

    let c = &(*confirmation);
    c_uint::try_from(c.timestamp).unwrap_or(0)
}

/// Frees memory for a Confirmation
///
/// ## Arguments
/// `address` - The pointer of a Confirmation
///
/// ## Returns
/// `()` - Does not return a value, equivalent to void in C
///
/// # Safety
/// None
#[no_mangle]
pub unsafe extern "C" fn destroy_confirmation(address: *mut Confirmation) {
    if !address.is_null() {
        drop(Box::from_raw(address))
    }
}

#[cfg(test)]
mod test {
    use tari_contacts::contacts_service::types::{Confirmation, MessageBuilder};
    use tari_utilities::epoch_time::EpochTime;

    use crate::{
        confirmation::{destroy_confirmation, read_confirmation_message_id, read_confirmation_timestamp},
        types::{chat_byte_vector_get_at, chat_byte_vector_get_length},
    };

    #[test]
    fn test_reading_from_confrimation() {
        let message_id = MessageBuilder::new().build().message_id;
        let timestamp = EpochTime::now().as_u64();
        let confirmation = Confirmation {
            message_id: message_id.clone(),
            timestamp,
        };

        let confirmation_ptr = Box::into_raw(Box::new(confirmation));
        let error_out = Box::into_raw(Box::new(0));

        unsafe {
            let id_byte_vec = read_confirmation_message_id(confirmation_ptr, error_out);
            let len = chat_byte_vector_get_length(id_byte_vec, error_out);

            let mut read_id = vec![];
            for i in 0..len {
                read_id.push(chat_byte_vector_get_at(id_byte_vec, i, error_out));
            }

            assert_eq!(message_id, read_id)
        }

        unsafe {
            let read_timestamp = read_confirmation_timestamp(confirmation_ptr, error_out);
            assert_eq!(timestamp, u64::from(read_timestamp))
        }

        unsafe { destroy_confirmation(confirmation_ptr) }
    }
}
