use crate::trace::{Batch, BatchReader, Cursor};

pub struct MmapBatch {

}

impl BatchReader<Vec<u8>, Vec<u8>, u64, isize> for MmapBatch {
    type Cursor = MmapCursor;

    fn cursor(&self) -> Self::Cursor {
        MmapCursor
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn description(&self) -> &crate::trace::Description<u64> {
        todo!()
    }
}

pub struct MmapCursor;

impl Cursor<Vec<u8>, Vec<u8>, u64, isize> for MmapCursor {
    type Storage = MmapBatch;

    fn key_valid(&self, storage: &Self::Storage) -> bool {
        todo!()
    }

    fn val_valid(&self, storage: &Self::Storage) -> bool {
        todo!()
    }

    fn key<'a>(&self, storage: &'a Self::Storage) -> &'a [u8] {
        todo!()
    }

    fn val<'a>(&self, storage: &'a Self::Storage) -> &'a Vec<u8> {
        todo!()
    }

    fn map_times<L: FnMut(&u64, &isize)>(&mut self, storage: &Self::Storage, logic: L) {
        todo!()
    }

    fn step_key(&mut self, storage: &Self::Storage) {
        todo!()
    }

    fn seek_key(&mut self, storage: &Self::Storage, key: &[u8]) {
        todo!()
    }

    fn step_val(&mut self, storage: &Self::Storage) {
        todo!()
    }

    fn seek_val(&mut self, storage: &Self::Storage, val: &Vec<u8>) {
        todo!()
    }

    fn rewind_keys(&mut self, storage: &Self::Storage) {
        todo!()
    }

    fn rewind_vals(&mut self, storage: &Self::Storage) {
        todo!()
    }
}