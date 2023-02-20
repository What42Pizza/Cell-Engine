use std::{ptr, hash::BuildHasher};
use hashbrown::{hash_map::DefaultHashBuilder, HashMap, Equivalent};
use std::hash::Hash;





// TODO: check on https://github.com/rust-lang/hashbrown/issues/332
pub trait HashMapAdditions<K, V, Q: ?Sized + Hash + Equivalent<K>> {
    fn get_many_mut_vec<'b> (&'b mut self, keys: &[&Q]) -> Option<Vec<&'b mut V>>;
    fn build_hashes (&self, keys: &[&Q]) -> Vec<u64>;
    fn make_hash (val: &Q) -> u64;
}



impl<'a, K: 'a, V, Q: ?Sized + Hash + Equivalent<K>> HashMapAdditions<K, V, Q> for HashMap<K, V> {

    fn get_many_mut_vec<'b> (&'b mut self, keys: &[&Q]) -> Option<Vec<&'b mut V>> {
        let hashes = Self::build_hashes(self, keys);
        let mut output: Vec<&mut (K, V)> = Vec::with_capacity(keys.len());

        let raw_table = self.raw_table();
        unsafe {
            for (i, &hash) in hashes.iter().enumerate() {
                let cur = raw_table.find(hash, |(k, _)| keys[i].equivalent(k))?;
                output.push(cur.as_mut());
            }
        }

        for (i, cur) in output.iter().enumerate() {
            if output[..i].iter().any(|prev| ptr::eq::<(K, V)>(*prev, *cur)) {
                return None;
            }
        }

        let output = output.into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<&mut V>>();

        Some(output)
    }

    fn build_hashes (&self, keys: &[&Q]) -> Vec<u64> {
        let mut hashes = Vec::with_capacity(keys.len());
        for &key in keys {
            hashes.push(Self::make_hash(key));
        }
        hashes
    }

    fn make_hash (val: &Q) -> u64 {
        use core::hash::Hasher;
        let mut state = DefaultHashBuilder::default().build_hasher();
        val.hash(&mut state);
        state.finish()
    }

}





/*
pub struct W<T> (T);

pub fn wrap<T> (value: T) -> W<T> {
    W (value)
}

impl<T> Deref for W<T> {
    type Target = T;
    #[inline(always)]
    fn deref (&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for W<T> {
    #[inline(always)]
    fn deref_mut (&mut self) -> &mut T {
        &mut self.0
    }
}





impl<'a, K: 'a, V> W<HashMap<K, V>> {

    pub fn get_many_mut_vec<'b, Q: ?Sized + Hash + Equivalent<K>> (&'b mut self, keys: Vec<&Q>) -> Option<Vec<&'b mut V>> {
        let hashes = self.build_hashes(&keys);
        let mut output: Vec<&mut (K, V)> = vec!();

        let raw_table = self.raw_table();
        unsafe {
            for (i, &hash) in hashes.iter().enumerate() {
                let cur = raw_table.find(hash, |(k, v)| keys[i].equivalent(k))?;
                output.push(cur.as_mut());
            }
        }

        for (i, cur) in output.iter().enumerate() {
            if output[..i].iter().any(|prev| ptr::eq::<(K, V)>(*prev, *cur)) {
                return None;
            }
        }

        let output = output.into_iter()
            .map(|(k, v)| v)
            .collect::<Vec<&mut V>>();

        Some(output)
    }

    pub fn build_hashes<Q: ?Sized + Hash + Equivalent<K>> (&self, keys: &Vec<&Q>) -> Vec<u64> {
        let mut hashes = Vec::with_capacity(keys.len());
        for i in 0..keys.len() {
            hashes.push(Self::make_hash(keys[i]));
        }
        hashes
    }

    pub fn make_hash<Q: ?Sized + Hash + Equivalent<K>> (val: &Q) -> u64 {
        use core::hash::Hasher;
        let mut state = DefaultHashBuilder::default().build_hasher();
        val.hash(&mut state);
        state.finish()
    }

}
*/
