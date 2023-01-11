import matplotlib.pyplot as plt
import numpy as np
from matplotlib.animation import FuncAnimation

def animate(_):
    graph_data = np.genfromtxt('train_output.log')
    datax1 = graph_data[:, 0]
    datax2 = graph_data[:, 1]
    plt.clf()
    plt.subplot(1,2,1)
    plt.plot(datax1)
    plt.title("Train Err")
    plt.subplot(1,2,2)
    plt.plot(datax2)
    plt.title("Test Err")


ani = FuncAnimation(plt.gcf(), animate, 1000)
plt.tight_layout()
plt.show()
