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
use tch::{Tensor, Device, CModule, nn::Module};
use anyhow::Result;

use crate::board::Position;

const BOARD_BITS: usize = 48;
const FEATURES: usize = 2 * BOARD_BITS + 1 + 1;

#[derive(Debug)]
pub struct Nnue {
    /// the network to use to evaluate.
    net: CModule,

    /// array to construct the tensor.
    tensor_arr: [f32; FEATURES],

    /// the tensor to input.
    tensor: Tensor
}


impl Nnue {
    /// Loads a new network from a file.
    pub fn new(modelfile: &Path, device: Device) -> Result<Self> {
        let net = tch::jit::CModule::load_on_device(modelfile, device)?;
        let tensor = Tensor::new();
        let tensor_arr = [0.; FEATURES];

        Ok(Self { net, tensor, tensor_arr })
    }

    fn update(
        &mut self,
        p0: Position,
        p1: Position,
        p2mv: u8,
        moves: u32) {

        for i in 0..BOARD_BITS {
            self.tensor_arr[i] = ((p0 >> i) & 1) as f32;
        }
        for (sh, i) in (BOARD_BITS..2*BOARD_BITS).enumerate() {
            self.tensor_arr[i] = ((p1 >> sh) & 1) as f32;
        }

        self.tensor_arr[FEATURES - 2] = p2mv as f32;
        self.tensor_arr[FEATURES - 1] = moves as f32;

        self.tensor = Tensor::of_slice(&self.tensor_arr);
    }


    pub fn evaluate(
        &mut self,
        p0: Position,
        p1: Position,
        p2mv: u8,
        moves: u32) -> isize {

        self.update(p0, p1, p2mv, moves);
        let value = f32::from(self.net.forward(&self.tensor));

        return value.round() as isize
    }
}
