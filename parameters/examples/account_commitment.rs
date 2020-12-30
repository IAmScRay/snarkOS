// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

use snarkos_errors::algorithms::CommitmentError;
use snarkos_models::{algorithms::CommitmentScheme, dpc::DPCComponents};
use snarkos_utilities::{bytes::ToBytes, to_bytes};
use snarkvm_dpc::base_dpc::instantiated::Components;

use rand::thread_rng;
use std::path::PathBuf;

mod utils;
use utils::store;

pub fn setup<C: DPCComponents>() -> Result<Vec<u8>, CommitmentError> {
    let rng = &mut thread_rng();
    let account_commitment = <C::AccountCommitment as CommitmentScheme>::setup(rng);
    let account_commitment_parameters = account_commitment.parameters();
    let account_commitment_parameters_bytes = to_bytes![account_commitment_parameters]?;

    let size = account_commitment_parameters_bytes.len();
    println!("account_commitment.params\n\tsize - {}", size);
    Ok(account_commitment_parameters_bytes)
}

pub fn main() {
    let bytes = setup::<Components>().unwrap();
    let filename = PathBuf::from("account_commitment.params");
    let sumname = PathBuf::from("account_commitment.checksum");
    store(&filename, &sumname, &bytes).unwrap();
}
