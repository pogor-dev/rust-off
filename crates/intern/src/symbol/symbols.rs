//! Module defining all known symbols required by the rest of rust-analyzer.
#![allow(non_upper_case_globals)]

use std::hash::{BuildHasherDefault, Hash as _, Hasher as _};

use dashmap::{DashMap, SharedValue};
use rustc_hash::FxHasher;

use crate::{
    symbol::{SymbolProxy, TaggedArcPtr},
    Symbol,
};

macro_rules! define_symbols {
    (@WITH_NAME: $($alias:ident = $value:literal,)* @PLAIN: $($name:ident,)*) => {
        // We define symbols as both `const`s and `static`s because some const code requires const symbols,
        // but code from before the transition relies on the lifetime of the predefined symbols and making them
        // `const`s make it error (because now they're temporaries). In the future we probably should only
        // use consts.

        /// Predefined symbols as `const`s (instead of the default `static`s).
        pub mod consts {
            use super::{Symbol, TaggedArcPtr};

            // The strings should be in `static`s so that symbol equality holds.
            $(
                pub const $name: Symbol = {
                    static SYMBOL_STR: &str = stringify!($name);
                    Symbol { repr: TaggedArcPtr::non_arc(&SYMBOL_STR) }
                };
            )*
            $(
                pub const $alias: Symbol = {
                    static SYMBOL_STR: &str = $value;
                    Symbol { repr: TaggedArcPtr::non_arc(&SYMBOL_STR) }
                };
            )*
        }

        $(
            pub static $name: Symbol = consts::$name;
        )*
        $(
            pub static $alias: Symbol = consts::$alias;
        )*


        pub(super) fn prefill() -> DashMap<SymbolProxy, (), BuildHasherDefault<FxHasher>> {
            let mut dashmap_ = <DashMap<SymbolProxy, (), BuildHasherDefault<FxHasher>>>::with_hasher(BuildHasherDefault::default());

            let hash_thing_ = |hasher_: &BuildHasherDefault<FxHasher>, it_: &SymbolProxy| {
                let mut hasher_ = std::hash::BuildHasher::build_hasher(hasher_);
                it_.hash(&mut hasher_);
                hasher_.finish()
            };
            {
                $(

                    let proxy_ = SymbolProxy($name.repr);
                    let hash_ = hash_thing_(dashmap_.hasher(), &proxy_);
                    let shard_idx_ = dashmap_.determine_shard(hash_ as usize);
                    dashmap_.shards_mut()[shard_idx_].get_mut().raw_entry_mut().from_hash(hash_, |k| k == &proxy_).insert(proxy_, SharedValue::new(()));
                )*
                $(

                    let proxy_ = SymbolProxy($alias.repr);
                    let hash_ = hash_thing_(dashmap_.hasher(), &proxy_);
                    let shard_idx_ = dashmap_.determine_shard(hash_ as usize);
                    dashmap_.shards_mut()[shard_idx_].get_mut().raw_entry_mut().from_hash(hash_, |k| k == &proxy_).insert(proxy_, SharedValue::new(()));
                )*
            }
            dashmap_
        }
    };
}

// TODO: check the values
define_symbols! {
    @WITH_NAME:
    __empty = "",

    @PLAIN:
}
