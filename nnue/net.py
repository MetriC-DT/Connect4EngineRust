#!/usr/bin/env python

import torch, os
import numpy as np
from torch import nn
from torch.optim import Optimizer
from torch.utils.data import DataLoader

# Hyperparameters
LEARNING_RATE = 1e-4
LOSS_FN = nn.functional.mse_loss
EPOCHS = 1000
BATCH_SIZE = 128

# Number of features the network takes in as inputs.
NUM_FEATURES = 2 * 42 + 1
L0 = 32
L1 = 32

# only the bits [0..5, 7..12, 14..19, 21..26, 28..33, 35..40, 42..47] are relevant.
# Subtracted 63 since notation is little endian.
indices = 63 - torch.cat((
    torch.arange(0, 6),
    torch.arange(7, 13),
    torch.arange(14,20),
    torch.arange(21,27),
    torch.arange(28,34),
    torch.arange(35,41),
    torch.arange(42,48),
))


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


    def forward(self, p0_ft, p1_ft, stm):
        """
        p0_ft: features for player 0 (first to move)
        p1_ft: features for player 1 (2nd to move)
        stm: side to move (either 1 or 0)
        """
        stm = np.array(stm).reshape((len(stm), 1))
        p0 = np.array(p0_ft, dtype='>u8').reshape((len(p0_ft), 1))
        p1 = np.array(p1_ft, dtype='>u8').reshape((len(p1_ft), 1))
        p0_v = np.unpackbits(p0.view(np.uint8), axis=1)
        p1_v = np.unpackbits(p1.view(np.uint8), axis=1)

        stm_t = torch.from_numpy(stm)
        p0_t = torch.from_numpy(p0_v)
        p1_t = torch.from_numpy(p1_v)

        p0_slice = p0_t[:,indices]
        p1_slice = p1_t[:,indices]

        # print(p0_slice)
        # print(p1_slice)

        inputs = torch.cat((p0_slice, p1_slice, stm_t), dim=1).to(torch.float32)
        return self.seq_stack(inputs)



def load_model(path: str, device: str):
    """ Loads the model if it exists, otherwise, returns a new one. """
    model = Net()
    optimizer = torch.optim.Adam(model.parameters(), lr=LEARNING_RATE)

    if os.path.exists(path):
        ckpt = torch.load(path)
        model.load_state_dict(ckpt['model'])
        # optimizer.load_state_dict(ckpt['optimizer'])
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
    for batch, (p0, p1, stm, expected) in enumerate(dataloader):
        predicted = model(p0, p1, stm)
        err = loss_fn(predicted, expected)
        opt_fn.zero_grad()
        err.backward()
        opt_fn.step()
        train_err += err.item()
        # if batch % 4 == 0:
        #     print(f'Batch {batch} Err {err.item()}')

    print(f'TRAIN ERR: {train_err}')
    return train_err


def save_model(file: str, model: Net, optimizer: Optimizer, loss):
    """ Saves the model and the relevant functions to the specified file. """

    model_state_dict = model.state_dict()
    opt_state_dict = optimizer.state_dict()
    savedata = { 'model': model_state_dict, 'optimizer': opt_state_dict, 'loss': loss }
    torch.save(savedata, file)
    return


def iterate_train(model, dataloader, loss_fn, opt_fn):
    for t in range(EPOCHS):
        print(f"EPOCH {t+1} -------------------------------------------")
        train_model(model, dataloader, loss_fn, opt_fn)
