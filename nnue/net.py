#!/usr/bin/env python

import torch, os, time
import numpy as np
from torch import nn
from torch.functional import Tensor
from torch.optim import Optimizer
from torch.utils.data import DataLoader

# Hyperparameters
LEARNING_RATE = 1e-4
LOSS_FN = nn.functional.mse_loss
EPOCHS = 500
BATCH_SIZE = 1024
OUT_LOG = "train_output.log"

# Number of features the network takes in as inputs.
BOARD_BITS = 48
NUM_FEATURES = 2 * BOARD_BITS + 1 + 1
L0 = 16
L1 = 16

# only the bits [0..5, 7..12, 14..19, 21..26, 28..33, 35..40, 42..47] are relevant.
# Subtracted 63 since notation is little endian.
indices = 63 - torch.cat((
    torch.arange(0, BOARD_BITS),
))

def get_tensor(p0_ft, p1_ft, stm, moves):
    """
    p0_ft: features for player 0 (first to move)
    p1_ft: features for player 1 (2nd to move)
    stm: side to move (either 1 or 0)
    moves: number of moves made.
    """
    stm = np.array(stm).reshape((len(stm), 1))
    moves = np.array(moves).reshape((len(moves), 1))
    p0 = np.array(p0_ft, dtype='>u8').reshape((len(p0_ft), 1))
    p1 = np.array(p1_ft, dtype='>u8').reshape((len(p1_ft), 1))
    p0_v = np.unpackbits(p0.view(np.uint8), axis=1)
    p1_v = np.unpackbits(p1.view(np.uint8), axis=1)

    stm_t = torch.from_numpy(stm)
    p0_t = torch.from_numpy(p0_v)
    p1_t = torch.from_numpy(p1_v)

    p0_slice = p0_t[:,indices]
    p1_slice = p1_t[:,indices]
    stm_slice = stm_t
    moves_slice = torch.from_numpy(moves)

    inputs = torch.cat((p0_slice, p1_slice, stm_slice, moves_slice), dim=1).to(torch.float32)
    return inputs


class Net(nn.Module):
    """ The neural network to train. """

    def __init__(self) -> None:
        super(Net, self).__init__()
        self.seq_stack = nn.Sequential(
            nn.Linear(NUM_FEATURES, L0),
            nn.ReLU(),
            nn.Linear(L0, L1),
            nn.ReLU(),
            nn.Linear(L1, 1)
        )
        return

    def forward(self, inputs: Tensor):
        """
        Inputs are tensors of size (98, 1).
        p0 - inputs[0:47]
        p1 - inputs[48:95]
        stm - inputs[96]
        moves - inputs[97]
        """
        return self.seq_stack(inputs)


def load_model(path: str, device: str):
    """ Loads the model if it exists, otherwise, returns a new one. """
    model = Net()
    optimizer = torch.optim.Adam(model.parameters())

    if os.path.exists(path):
        ckpt = torch.load(path)
        model.load_state_dict(ckpt['model'])
        optimizer.load_state_dict(ckpt['optimizer'])
        loss = ckpt['loss']
    else:
        loss = LOSS_FN

    model = model.to(device)
    print(model.eval())
    return (model, optimizer, loss)


def train_model(model: Net,
                dataloader: DataLoader,
                loss_fn,
                opt_fn: Optimizer):

    train_err = 0
    for (p0, p1, stm, moves, expected) in dataloader:
        tensor = get_tensor(p0, p1, stm, moves)
        predicted = model(tensor)
        err = loss_fn(predicted, expected)
        opt_fn.zero_grad()
        err.backward()
        opt_fn.step()
        train_err += err.item()
        # if batch % 4 == 0:
        #     print(f'Batch {batch} Err {err.item()}')

    avg_error = train_err * BATCH_SIZE / len(dataloader)
    print(f'TRAIN ERR: {avg_error}')
    return avg_error

def test_model(model: Net,
               dataloader: DataLoader,
               loss_fn):

    test_err = 0
    for (p0, p1, stm, moves, expected) in dataloader:
        tensor = get_tensor(p0, p1, stm, moves)
        predicted = model(tensor)
        err = loss_fn(predicted, expected)
        test_err += err.item()

    avg_error = test_err * BATCH_SIZE / len(dataloader)
    print(f'TEST ERR: {avg_error}')
    return avg_error


def save_model(file: str, model: Net, optimizer: Optimizer, loss):
    """ Saves the model and the relevant functions to the specified file. """

    model_state_dict = model.state_dict()
    opt_state_dict = optimizer.state_dict()
    savedata = { 'model': model_state_dict, 'optimizer': opt_state_dict, 'loss': loss }
    torch.save(savedata, file)
    return


def iterate_train(model, train_data, test_data, loss_fn, opt_fn):
    with open(OUT_LOG, 'a') as f:
        for t in range(EPOCHS):
            print(f"EPOCH {t+1} -------------------------------------------")
            start = time.perf_counter()
            train_err = train_model(model, train_data, loss_fn, opt_fn)
            test_err = test_model(model, test_data, loss_fn)
            f.write(f"{train_err} {test_err}\n")
            f.flush()
            end = time.perf_counter()
            print(f"TIME ELAPSED = {(end - start) * 1e3}ms")
