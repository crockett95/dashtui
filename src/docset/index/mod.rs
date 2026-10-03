//! Reading a docset's search index (`Contents/Resources/docSet.dsidx`).
//!
//! Each on-disk schema gets its own submodule. M4 adds the second schema
//! (Core Data) and the `IndexReader` trait that puts both behind one interface.

pub mod search_index;
