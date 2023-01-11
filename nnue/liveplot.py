import matplotlib.pyplot as plt
import numpy as np
from matplotlib.animation import FuncAnimation

def animate(_):
    graph_data = np.genfromtxt('train_output.log')
    datax1 = graph_data[:, 0]
    datax2 = graph_data[:, 1]
    plt.clf()
    plt.plot(datax1, label='train')
    plt.plot(datax2, label='test')
    plt.legend()


ani = FuncAnimation(plt.gcf(), animate, 1000)
plt.tight_layout()
plt.show()
