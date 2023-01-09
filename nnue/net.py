#!/usr/bin/env python

import torch
from torch import nn, Tensor

# Number of features the network takes in as inputs.
NUM_FEATURES = 2 * 42 + 1
L0 = 16
L1 = 8

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


    def forward(self, p0_ft: Tensor, p1_ft: Tensor, stm: Tensor):
        """
        p0_ft: features for player 0 (first to move)
        p1_ft: features for player 1 (2nd to move)
        stm: side to move (either 1 or 0)
        """
        inputs = torch.cat((p0_ft, p1_ft, stm), dim=1)
        print(inputs)
        return self.seq_stack(inputs)
