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
EPOCHS = 50
BATCH_SIZE = 1024
OUT_LOG = "train_output.log"

# Number of features the network takes in as inputs.
BOARD_BITS = 48
NUM_FEATURES = 2 * BOARD_BITS + 1 + 1
L0 = 64
L1 = 32
L2 = 32
L3 = 32
L4 = 16

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
            nn.Linear(L1, L2),
            nn.ReLU(),
            nn.Linear(L2, L3),
            nn.ReLU(),
            nn.Linear(L3, 1)
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
    optimizer = torch.optim.Adam(model.parameters(), lr=LEARNING_RATE)
    loss = LOSS_FN

    if os.path.exists(path):
        ckpt = torch.load(path)
        model.load_state_dict(ckpt)

    model = model.to(device)
    print(model.eval())
    return (model, optimizer, loss)


def train_model(model: Net,
                dataloader: DataLoader,
                loss_fn,
                opt_fn: Optimizer):

    correct = 0
    for (p0, p1, stm, moves, expected) in dataloader:
        tensor = get_tensor(p0, p1, stm, moves)
        predicted = model(tensor)
        err = loss_fn(predicted, expected)
        correct += count_correct(predicted, expected)
        opt_fn.zero_grad()
        err.backward()
        opt_fn.step()

    train_accuracy = correct / len(dataloader.dataset)
    print(f'TRAIN ACC: {train_accuracy}')
    return train_accuracy


def count_correct(pred: Tensor, expected: Tensor):
    predicted_round = torch.round(pred)
    return torch.count_nonzero(predicted_round == expected)


def test_model(model: Net,
               dataloader: DataLoader):

    correct = 0
    for (p0, p1, stm, moves, expected) in dataloader:
        tensor = get_tensor(p0, p1, stm, moves)
        predicted = model(tensor)
        correct += count_correct(predicted, expected)

    test_accuracy = correct / len(dataloader.dataset)
    print(f'TEST ACC: {test_accuracy}')
    return test_accuracy


def save_model(file: str, model: Net):
    """ Saves the model to the specified file. """
    model_state_dict = model.state_dict()
    torch.save(model_state_dict, file)
    return


def iterate_train(model, train_data, test_data, loss_fn, opt_fn):
    with open(OUT_LOG, 'a') as f:
        for t in range(EPOCHS):
            print(f"EPOCH {t+1} -------------------------------------------")
            start = time.perf_counter()
            train_acc = train_model(model, train_data, loss_fn, opt_fn)
            test_acc = test_model(model, test_data)
            f.write(f"{train_acc} {test_acc}\n")
            f.flush()
            end = time.perf_counter()
            print(f"TIME ELAPSED = {(end - start) * 1e3:0.2f}ms")
