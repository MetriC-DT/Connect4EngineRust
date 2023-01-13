#!/usr/bin/env python

import sqlite3, sys, torch, os
import net
from torch.utils.data import DataLoader, TensorDataset
from torch import Tensor

def get_data(datafile: str):
    conn = sqlite3.connect(datafile)
    cur = conn.cursor()
    result = cur.execute('SELECT * from positions')
    (_, p2mv, moves, p0, p1, eval) = list(zip(*result))
    conn.close()

    p2mv = Tensor(p2mv).reshape((len(p2mv), 1))
    p0 = Tensor(p0).reshape((len(p0), 1))
    p1 = Tensor(p1).reshape((len(p1), 1))
    eval = Tensor(eval).reshape((len(eval), 1))
    moves = Tensor(moves).reshape((len(moves), 1))
    return (p0, p1, p2mv, moves, eval)


if __name__ == '__main__':
    if len(sys.argv) != 4:
        print("Usage: main.py <MODEL_FILE> <TRAINING_DB> <TEST_DB>")
        exit(-1)

    (_, modelfile, traindata, testdata) = sys.argv

    if not os.path.exists(traindata) or not os.path.exists(testdata):
        print("Database path doesn't exist")
        exit(-1)

    device = "cuda" if torch.cuda.is_available() else "cpu"
    model, opt, loss = net.load_model(modelfile, device)
    training = get_data(traindata)
    testing = get_data(testdata)

    train_dataset = TensorDataset(*training)
    test_dataset = TensorDataset(*testing)
    train_dl = DataLoader(train_dataset, batch_size=net.BATCH_SIZE, num_workers=8)
    test_dl = DataLoader(test_dataset, num_workers=8, batch_size=len(test_dataset))

    if os.path.isfile(net.OUT_LOG):
        os.remove(net.OUT_LOG)

    # net.iterate_train(model, train_dl, test_dl, loss, opt)
    # net.save_model(modelfile, model)
    t = net.get_tensor([1<<(2*7)], [0], [1], [1])
    print(model(t))
    # sm = torch.jit.trace(model, t)
    # sm.save(f"export_{modelfile}")
