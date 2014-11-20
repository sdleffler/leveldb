extern crate key;
extern crate leveldb;
extern crate serialize;

#[allow(dead_code)]
mod utils;

#[cfg(test)]
mod comparator {
  use key::Key;
  use utils::{tmpdir, db_put_simple};
  use leveldb::database::{Database};
  use leveldb::iterator::Iterable;
  use leveldb::options::{Options,ReadOptions};
  use leveldb::comparator::Comparator;
  
  struct ReverseComparator<K>;

  impl<K: Key> Comparator<K> for ReverseComparator<K> {
    fn name(&self) -> *const u8 {
      "reverse".as_ptr()
    }
  
    fn compare(&self, a: &K, b: &K) -> Ordering {
      b.compare(a)
    }

  }

  #[test]
  fn test_comparator() {
    let comparator: ReverseComparator<int> = ReverseComparator::<int>;
    let mut opts = Options::new();
    opts.create_if_missing = true;
    let tmp = tmpdir("testdbs");
    let database = &mut Database::open(tmp.path().join("reverse_comparator"), opts, Some(comparator)).unwrap();
    db_put_simple(database, 1, &[1]);
    db_put_simple(database, 2, &[2]);

    let read_opts = ReadOptions::new();
    let mut iter = database.iter(read_opts);

    assert_eq!((2, vec![2]), iter.next().unwrap())
    assert_eq!((1, vec![1]), iter.next().unwrap())
  }
}
