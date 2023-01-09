#!/usr/bin/env python

import sqlite3, sys, torch, os
import net
from torch.utils.data import DataLoader, TensorDataset
from torch import Tensor


def get_data(datafile: str):
    conn = sqlite3.connect(datafile)
    cur = conn.cursor()
    result = cur.execute('SELECT * from positions')
    (_, p2mv, p0, p1, eval) = list(zip(*result))
    conn.close()

    p2mv = Tensor(p2mv).reshape((len(p2mv), 1))
    p0 = Tensor(p0).reshape((len(p0), 1))
    p1 = Tensor(p1).reshape((len(p1), 1))
    eval = Tensor(eval).reshape((len(eval), 1))
    return (p2mv, p0, p1, eval)


if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: main.py <MODEL_FILE> <DATABASE_FILE>")
        exit(-1)

    (_, modelfile, datafile) = sys.argv

    if not os.path.exists(datafile):
        print("Database path doesn't exist")
        exit(-1)

    device = "cuda" if torch.cuda.is_available() else "cpu"
    model, opt, loss = net.load_model(modelfile, device)
    (p2mv, p0, p1, eval) = get_data(datafile)

    dataset = TensorDataset(p0, p1, p2mv, eval)
    dataloader = DataLoader(dataset, batch_size=net.BATCH_SIZE, num_workers=4)

    net.iterate_train(model, dataloader, loss, opt)
    net.save_model(modelfile, model, opt, loss)
    # print(model(Tensor([1 << (6*7)]), Tensor([1 << (3*7)]), Tensor([0])))
