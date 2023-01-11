// Connect4EngineRust, a strong solver for the connect-4 board game.
// Copyright (C) 2023 Derick Tseng
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::path::Path;
use tch::{Tensor, Device, CModule};
use crate::board::{Position, SIZE};
use anyhow::Result;

#[derive(Debug)]
pub struct Nnue {
    /// saved player position (indexed 0 or 1 depending on current player).
    p_prev: [Position; 2],

    /// indices of set bits in player's positions to be used to construct the `input_tensor`.
    /// There can be at most 42 set bits in a board.
    /// Player 0 will put in values [0..41]
    /// Player 1 will put in values [42..83]
    indices: [usize; SIZE as usize],

    /// size of index. We need to insert in ascending order.
    index_sz: usize,

    /// the network to use to evaluate.
    net: CModule,

    /// Tensor to use as the input to the NNUE
    input_tensor: Tensor
}

impl Nnue {
    /// Loads a new network from a file.
    fn new(modelfile: &Path, device: Device) -> Result<Self> {
        let p_prev = [0, 0];
        let indices = [0; SIZE as usize];
        let index_sz = 0;
        let input_tensor = Tensor::new();

        let net = tch::jit::CModule::load_on_device(modelfile, device)?;

        Ok(Self { p_prev, indices, index_sz, net, input_tensor })
    }

    fn get_tensor(&mut self) -> &Tensor {
        &self.input_tensor
    }

    fn evaluate(&mut self) -> i8 {
        3
    }
}
