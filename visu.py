#!/usr/bin/env python3
import sys
import signal
import matplotlib.pyplot as plt
from util import *


def main():
    """
        Main of visu
    """
    signal.signal(signal.SIGINT, signal_handler)
    x, y = get_data_csv()
    line = get_gradient_csv()
    plt.plot(x, y, 'ro', label='data')
    x_estim = sorted(x)
    y_estim = [estimated_price(line['a'], _, line['b']) for _ in x_estim]
    plt.plot(x_estim, y_estim, 'b--', label='Estimation')
    plt.ylabel('Price (in euro)')
    plt.xlabel('Mileage (in km)')
    plt.grid(True)
    plt.title('Price = f(Mileage)')
    plt.show()


if __name__ == '__main__':
    main()
