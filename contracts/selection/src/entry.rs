// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    high_level::{load_cell_lock_hash, load_script, QueryIter},
};

use axon_types::{selection_reader as axon, Cursor};
use util::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();

    // extract omni and reward lock_hash from script_args
    let selection_args: axon::SelectionLockArgs = Cursor::from(args.to_vec()).into();
    let omni_lock_hash = selection_args.omni_lock_hash();
    let reward_type_id = selection_args.reward_type_id();

    // count omni and reward cells count
    let mut omni_cells_count = 0;
    let mut reward_cells_count = 0;

    // search omni and reward cells via ckb functions
    QueryIter::new(load_cell_lock_hash, Source::Input).for_each(|lock_hash| {
        if &lock_hash == omni_lock_hash.as_slice() {
            omni_cells_count += 1;
        } else if &lock_hash == reward_type_id.as_slice() {
            reward_cells_count += 1;
        }
    });

    debug!(
        "omni = {}, reward = {}",
        omni_cells_count, reward_cells_count
    );

    // sum of omni and reward must be 1
    if omni_cells_count + reward_cells_count != 1 {
        return Err(Error::OmniRewardCountError);
    }

    Ok(())
}
