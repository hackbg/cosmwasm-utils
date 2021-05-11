use serde::Serialize;
use serde::de::DeserializeOwned;
use cosmwasm_std::{ReadonlyStorage, StdError, StdResult, Storage, from_slice, to_vec};

pub fn save<T: Serialize, S: Storage>(storage: &mut S, key: &[u8], value: &T) -> StdResult<()> {
    storage.set(key, &to_vec(value)?);
    Ok(())
}

pub fn remove<S: Storage>(storage: &mut S, key: &[u8]) {
    storage.remove(key);
}

/// Returns a StdError::SerializeErr if there is no item with that key.
pub fn load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<T> {
    let result = storage.get(key).ok_or_else(||
        StdError::SerializeErr { 
            source: "load".into(),
            msg: "key not found".into(),
            backtrace: None
        }
    )?;

    from_slice(&result)
}

pub fn ns_save<T: Serialize, S: Storage>(storage: &mut S, namespace: &[u8], key: &[u8], value: &T) -> StdResult<()> {
    let key = concat(namespace, key);
    storage.set(&key, &to_vec(value)?);

    Ok(())
}

pub fn ns_remove<S: Storage>(storage: &mut S, namespace: &[u8], key: &[u8]) {
    let key = concat(namespace, key);
    storage.remove(&key);
}

/// Returns a StdError::SerializeErr if there is no item with that key in the namespace.
pub fn ns_load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, namespace: &[u8], key: &[u8]) -> StdResult<T> {
    let key = concat(namespace, key);

    load(storage, &key)
}

#[inline]
fn concat(namespace: &[u8], key: &[u8]) -> Vec<u8> {
    let mut k = namespace.to_vec();
    k.extend_from_slice(key);

    k
}
