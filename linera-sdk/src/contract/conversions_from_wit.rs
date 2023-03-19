// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Conversions from types generated by [`wit-bindgen-guest-rust`] to types declared in [`linera-sdk`].

use super::writable_system::{
    self as system, PollFindKeyValues, PollFindKeys, PollLock, PollReadKeyBytes, PollUnit,
};
use crate::{
    ApplicationId, BlockHeight, BytecodeId, CalleeContext, ChainId, CryptoHash, EffectContext,
    EffectId, OperationContext, Owner, Session, SessionId, SystemBalance,
};
use std::task::Poll;

impl From<super::OperationContext> for OperationContext {
    fn from(application_context: super::OperationContext) -> Self {
        OperationContext {
            chain_id: ChainId(application_context.chain_id.into()),
            authenticated_signer: application_context.authenticated_signer.map(Owner::from),
            height: BlockHeight(application_context.height),
            index: application_context.index,
        }
    }
}

impl From<super::EffectContext> for EffectContext {
    fn from(application_context: super::EffectContext) -> Self {
        EffectContext {
            chain_id: ChainId(application_context.chain_id.into()),
            authenticated_signer: application_context.authenticated_signer.map(Owner::from),
            height: BlockHeight(application_context.height),
            effect_id: application_context.effect_id.into(),
        }
    }
}

impl From<super::EffectId> for EffectId {
    fn from(effect_id: super::EffectId) -> Self {
        EffectId {
            chain_id: ChainId(effect_id.chain_id.into()),
            height: BlockHeight(effect_id.height),
            index: effect_id.index,
        }
    }
}

impl From<super::CalleeContext> for CalleeContext {
    fn from(application_context: super::CalleeContext) -> Self {
        CalleeContext {
            chain_id: ChainId(application_context.chain_id.into()),
            authenticated_signer: application_context.authenticated_signer.map(Owner::from),
            authenticated_caller_id: application_context
                .authenticated_caller_id
                .map(ApplicationId::from),
        }
    }
}

impl From<super::ApplicationId> for ApplicationId {
    fn from(application_id: super::ApplicationId) -> Self {
        ApplicationId {
            bytecode: BytecodeId(application_id.bytecode_id.into()),
            creation: application_id.creation.into(),
        }
    }
}

impl From<super::SessionId> for SessionId {
    fn from(session_id: super::SessionId) -> Self {
        SessionId {
            application_id: session_id.application_id.into(),
            kind: session_id.kind,
            index: session_id.index,
        }
    }
}

impl From<super::Session> for Session {
    fn from(session: super::Session) -> Self {
        Session {
            kind: session.kind,
            data: session.data,
        }
    }
}

impl From<super::CryptoHash> for Owner {
    fn from(crypto_hash: super::CryptoHash) -> Self {
        Owner(crypto_hash.into())
    }
}

impl From<super::CryptoHash> for CryptoHash {
    fn from(crypto_hash: super::CryptoHash) -> Self {
        CryptoHash::from([
            crypto_hash.part1,
            crypto_hash.part2,
            crypto_hash.part3,
            crypto_hash.part4,
        ])
    }
}

impl From<system::EffectId> for EffectId {
    fn from(effect_id: system::EffectId) -> Self {
        EffectId {
            chain_id: ChainId(effect_id.chain_id.into()),
            height: BlockHeight(effect_id.height),
            index: effect_id.index,
        }
    }
}

impl From<system::ApplicationId> for ApplicationId {
    fn from(application_id: system::ApplicationId) -> Self {
        ApplicationId {
            bytecode: BytecodeId(application_id.bytecode_id.into()),
            creation: application_id.creation.into(),
        }
    }
}

impl From<system::CryptoHash> for CryptoHash {
    fn from(hash_value: system::CryptoHash) -> Self {
        CryptoHash::from([
            hash_value.part1,
            hash_value.part2,
            hash_value.part3,
            hash_value.part4,
        ])
    }
}

impl From<system::SystemBalance> for SystemBalance {
    fn from(balance: system::SystemBalance) -> Self {
        let value = ((balance.upper_half as u128) << 64) | (balance.lower_half as u128);
        SystemBalance(value)
    }
}

impl From<PollLock> for Poll<bool> {
    fn from(poll_lock: PollLock) -> Poll<bool> {
        match poll_lock {
            PollLock::ReadyLocked => Poll::Ready(true),
            PollLock::ReadyNotLocked => Poll::Ready(false),
            PollLock::Pending => Poll::Pending,
        }
    }
}

impl From<system::CallResult> for (Vec<u8>, Vec<SessionId>) {
    fn from(call_result: system::CallResult) -> (Vec<u8>, Vec<SessionId>) {
        let value = call_result.value;

        let sessions = call_result
            .sessions
            .into_iter()
            .map(SessionId::from)
            .collect();

        (value, sessions)
    }
}

impl From<system::SessionId> for SessionId {
    fn from(session_id: system::SessionId) -> SessionId {
        SessionId {
            application_id: session_id.application_id.into(),
            kind: session_id.kind,
            index: session_id.index,
        }
    }
}

impl From<PollReadKeyBytes> for Poll<Option<Vec<u8>>> {
    fn from(poll_read_key_bytes: PollReadKeyBytes) -> Self {
        match poll_read_key_bytes {
            PollReadKeyBytes::Ready(bytes) => Poll::Ready(bytes),
            PollReadKeyBytes::Pending => Poll::Pending,
        }
    }
}

impl From<PollFindKeys> for Poll<Vec<Vec<u8>>> {
    fn from(poll_find_keys: PollFindKeys) -> Self {
        match poll_find_keys {
            PollFindKeys::Ready(keys) => Poll::Ready(keys),
            PollFindKeys::Pending => Poll::Pending,
        }
    }
}

impl From<PollFindKeyValues> for Poll<Vec<(Vec<u8>, Vec<u8>)>> {
    fn from(poll_find_key_values: PollFindKeyValues) -> Self {
        match poll_find_key_values {
            PollFindKeyValues::Ready(key_values) => Poll::Ready(key_values),
            PollFindKeyValues::Pending => Poll::Pending,
        }
    }
}

impl From<PollUnit> for Poll<()> {
    fn from(poll_write_batch: PollUnit) -> Self {
        match poll_write_batch {
            PollUnit::Ready => Poll::Ready(()),
            PollUnit::Pending => Poll::Pending,
        }
    }
}
