#!/usr/bin/env python

from net import Net
import torch
import os
import sqlite3
import sys

# General Hyperparameters
LEARNING_RATE = 1e-4
LOSS_FN = torch.nn.CrossEntropyLoss()


def load_model(path: str, device: str):
    """
    Loads the model if it exists, otherwise, returns a new one.
    """
    if os.path.exists(path):
        load_data = torch.load('model.pth')
        model = load_data['model']
        optimizer = load_data['optimizer']
        loss = load_data['loss']
    else:
        model = Net()
        optimizer = torch.optim.Adam(model.parameters(), lr=LEARNING_RATE)
        loss = LOSS_FN

    model = model.to(device)
    print(model.eval())
    return (model, optimizer, loss)


def train_model(model, inputs, expected):
    return


def save_model(model, optimizer, loss):
    savedata = {
        'model'     : model,
        'optimizer' : optimizer,
        'loss'      : loss,
    }
    torch.save(savedata, modelfile)
    return

def get_data(datafile: str):
    conn = sqlite3.connect(datafile)
    cur = conn.cursor()
    result = cur.execute('SELECT * from positions')
    (_, p2mv, p0, p1, eval) = list(zip(*result))

    return (p2mv, p0, p1, eval)


if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: main.py <MODEL_FILE> <DATABASE_FILE>")
        exit(-1)

    (_, modelfile, datafile) = sys.argv

    device = "cuda" if torch.cuda.is_available() else "cpu"
    model = load_model(modelfile, device)
    (p2mv, p0, p1, eval) = get_data(datafile)
