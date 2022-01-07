use super::super::{DbMap, DbXxx, DbXxxKeyType};
use super::{
    CheckFileDbMap, CountOfPerSize, DbXxxIntoIter, DbXxxIter, DbXxxIterMut, FileDbParams,
    FileDbXxxInner, Key, KeysCountStats, LengthStats, RecordSizeStats, Value,
};
use std::cell::RefCell;
use std::io::Result;
use std::path::Path;
use std::rc::Rc;

pub mod kt_dbbytes;
pub mod kt_dbint;
pub mod kt_dbstring;

pub use kt_dbbytes::{DbBytes, FileDbMapDbBytes};
pub use kt_dbint::{DbInt, FileDbMapDbInt};
pub use kt_dbstring::{DbString, FileDbMapString};

/// DbMap in a file database.
#[derive(Debug, Clone)]
pub struct FileDbMap<KT: DbXxxKeyType>(Rc<RefCell<FileDbXxxInner<KT>>>);

impl<KT: DbXxxKeyType> FileDbMap<KT> {
    pub(crate) fn open<P: AsRef<Path>>(
        path: P,
        ks_name: &str,
        params: FileDbParams,
    ) -> Result<FileDbMap<KT>> {
        Ok(Self(Rc::new(RefCell::new(
            FileDbXxxInner::<KT>::open_with_params(path, ks_name, params)?,
        ))))
    }
    #[inline]
    pub fn is_dirty(&self) -> bool {
        RefCell::borrow(&self.0).is_dirty()
    }
}

/// for debug
impl<KT: DbXxxKeyType + std::fmt::Display> CheckFileDbMap for FileDbMap<KT> {
    #[cfg(feature = "htx")]
    fn ht_size_and_count(&self) -> Result<(u64, u64)> {
        RefCell::borrow(&self.0).ht_size_and_count()
    }
    /// convert the index node tree to graph string for debug.
    fn graph_string(&self) -> Result<String> {
        RefCell::borrow(&self.0).graph_string()
    }
    /// convert the index node tree to graph string for debug.
    fn graph_string_with_key_string(&self) -> Result<String> {
        RefCell::borrow_mut(&self.0).graph_string_with_key_string()
    }
    /// check the index node tree is balanced
    fn is_balanced(&self) -> Result<bool> {
        RefCell::borrow(&self.0).is_balanced()
    }
    /// check the index node tree is multi search tree
    fn is_mst_valid(&self) -> Result<bool> {
        RefCell::borrow(&self.0).is_mst_valid()
    }
    /// check the index node except the root and leaves of the tree has branches of hm or more.
    fn is_dense(&self) -> Result<bool> {
        RefCell::borrow(&self.0).is_dense()
    }
    /// get the depth of the index node.
    fn depth_of_node_tree(&self) -> Result<u64> {
        RefCell::borrow(&self.0).depth_of_node_tree()
    }
    /// count of the free node
    fn count_of_free_node(&self) -> Result<CountOfPerSize> {
        RefCell::borrow(&self.0).count_of_free_node()
    }
    /// count of the free key record
    fn count_of_free_key_record(&self) -> Result<CountOfPerSize> {
        RefCell::borrow(&self.0).count_of_free_key_record()
    }
    /// count of the free value record
    fn count_of_free_value_record(&self) -> Result<CountOfPerSize> {
        RefCell::borrow(&self.0).count_of_free_value_record()
    }
    /// count of the used record and the used node
    fn count_of_used_node(&self) -> Result<(CountOfPerSize, CountOfPerSize, CountOfPerSize)> {
        RefCell::borrow(&self.0).count_of_used_node()
    }
    /// buffer statistics
    #[cfg(feature = "buf_stats")]
    fn buf_stats(&self) -> Vec<(String, i64)> {
        RefCell::borrow(&self.0).buf_stats()
    }
    /// key record size statistics
    fn key_record_size_stats(&self) -> Result<RecordSizeStats<Key>> {
        RefCell::borrow(&self.0).key_record_size_stats()
    }
    /// value record size statistics
    fn value_record_size_stats(&self) -> Result<RecordSizeStats<Value>> {
        RefCell::borrow(&self.0).value_record_size_stats()
    }
    /// keys count statistics
    fn keys_count_stats(&self) -> Result<KeysCountStats> {
        RefCell::borrow(&self.0).keys_count_stats()
    }
    /// key length statistics
    fn key_length_stats(&self) -> Result<LengthStats<Key>> {
        RefCell::borrow(&self.0).key_length_stats()
    }
    /// value length statistics
    fn value_length_stats(&self) -> Result<LengthStats<Value>> {
        RefCell::borrow(&self.0).value_length_stats()
    }
}

impl<KT: DbXxxKeyType> DbXxx<KT> for FileDbMap<KT> {
    #[inline]
    fn get_kt(&mut self, key: &KT) -> Result<Option<Vec<u8>>> {
        RefCell::borrow_mut(&self.0).get_kt(key)
    }
    #[inline]
    fn put_kt(&mut self, key: &KT, value: &[u8]) -> Result<()> {
        RefCell::borrow_mut(&self.0).put_kt(key, value)
    }
    #[inline]
    fn del_kt(&mut self, key: &KT) -> Result<Option<Vec<u8>>> {
        RefCell::borrow_mut(&self.0).del_kt(key)
    }
    #[inline]
    fn read_fill_buffer(&mut self) -> Result<()> {
        RefCell::borrow_mut(&self.0).read_fill_buffer()
    }
    #[inline]
    fn flush(&mut self) -> Result<()> {
        RefCell::borrow_mut(&self.0).flush()
    }
    #[inline]
    fn sync_all(&mut self) -> Result<()> {
        RefCell::borrow_mut(&self.0).sync_all()
    }
    #[inline]
    fn sync_data(&mut self) -> Result<()> {
        RefCell::borrow_mut(&self.0).sync_data()
    }
}

impl<KT: DbXxxKeyType> DbMap<KT> for FileDbMap<KT> {
    #[inline]
    fn iter(&self) -> DbXxxIter<KT> {
        DbXxxIter::new(self.0.clone()).unwrap()
    }
    #[inline]
    fn iter_mut(&mut self) -> DbXxxIterMut<KT> {
        DbXxxIterMut::new(self.0.clone()).unwrap()
    }
}

// impl trait: IntoIterator
impl<KT: DbXxxKeyType> IntoIterator for FileDbMap<KT> {
    type Item = (KT, Vec<u8>);
    type IntoIter = DbXxxIntoIter<KT>;
    //
    #[inline]
    fn into_iter(self) -> DbXxxIntoIter<KT> {
        DbXxxIntoIter::new(self.0).unwrap()
    }
}

impl<KT: DbXxxKeyType> IntoIterator for &FileDbMap<KT> {
    type Item = (KT, Vec<u8>);
    type IntoIter = DbXxxIter<KT>;
    //
    #[inline]
    fn into_iter(self) -> DbXxxIter<KT> {
        DbXxxIter::new(self.0.clone()).unwrap()
    }
}

impl<KT: DbXxxKeyType> IntoIterator for &mut FileDbMap<KT> {
    type Item = (KT, Vec<u8>);
    type IntoIter = DbXxxIterMut<KT>;
    //
    #[inline]
    fn into_iter(self) -> DbXxxIterMut<KT> {
        DbXxxIterMut::new(self.0.clone()).unwrap()
    }
}
