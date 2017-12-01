#!/usr/bin/env python3
import math
import argparse
import sys
from util import *
import signal
import matplotlib.pyplot as plt

def debug(message):
    """
        Verbosity for debug msg
    """
    if options.verbose > 2:
        print("\033[33m{:s}\033[0m".format(message))


def normal(message):
    """
        Verbosity for normal msg
    """
    if options.verbose > 0:
        print(message)


def success(message):
    """
        Verbosity for success msg
    """
    if options.verbose > 0:
        print("\033[32m{:s}\033[0m".format(message))
    else:
        print(message)


def verbose(message):
    """
        Verbosity for info msg
    """
    if options.verbose > 1:
        print("\033[38;5;247m{:s}\033[0m".format(message))


def error(message):
    """
        Verbosity for error msg
    """
    if options.verbose > 0:
        print("\033[31m{:s}\033[0m".format(message))


def optparse():
    """
        Parse arguments
    """
    parser = argparse.ArgumentParser()
    parser.add_argument('--verbose', '-v', dest='verbose', action='count', default=1,
                        help='Verbosity level. Can be accumulate to increase verbose')
    parser.add_argument('--quiet', '-q', dest='verbose', action='store_const', const=0,
                        help='Turn off verbosity')
    parser.add_argument('--iteration', '-i', action="store", dest="iter", type=int, default=500,
                        help='Change number of iteration. (default is 10)')
    parser.add_argument('--history', '-H', action="store_true", dest="history", default=False,
                        help='save history to futur display')
    parser.add_argument('--reuse', '-r', action="store_true", dest="reuse", default=False,
                        help='If set, will re-use known coeficient to update them')
    parser.add_argument('--learningRate', '-l', action="store", dest="rate", type=float, default=0.2,
                        help='Change learning coeficient. (default is 0.2)')
    return parser.parse_args()


def set_gradient_csv(gradient):
    try:
        with open('gradient.csv', 'w') as csvfile:
            for _ in 'ab':
                csvfile.write('{},{}\n'.format(_, gradient[_]))
    except:
        error('failed to save gradient')


def convert_data(val):
    maximum = max(val)
    minimum = min(val)
    return [(_ - minimum) / (maximum - minimum) for _ in val]

def gradient_descent(x, y, gradient):
    cpt = 0
    A, B = gradient['a'], gradient['b']
    length = len(x)
    history = []
    while cpt < options.iter:
        a, b = A, B
        sA = 0
        sB = 0
        for i, _ in enumerate(x):
            t = a * x[i] + b - y[i]
            sB += t
            sA += (t * x[i])
        A = A - options.rate * (sA / length)
        B = B - options.rate * (sB / length)
        cpt += 1
        history.append([A, B])
    gradient['a'], gradient['b'] = A, B
    return history

def main():
    """
        Main of visu
    """
    signal.signal(signal.SIGINT, signal_handler)
    global options
    options = optparse()
    if options.rate <= 0:
        error('{}: Wrong learning rate'.format(options.rate))
        sys.exit(1)
    x, y = get_data_csv()
    x = convert_data(x)
    y = convert_data(y)
    if options.reuse:
        gradient = get_gradient_csv()
    else:
        gradient = {'a':0, 'b':0}
    try:
        history = gradient_descent(x, y, gradient)
    except:
        error('Not able to analyze gradient descent')
        sys.exit(1)
    set_gradient_csv(gradient)
    try:
        if options.history:
            accuracy = []
            for hist in history:
                accuracy.append(sum([math.fabs(raw_estimated_price(hist[0], _, hist[1]) - y[i]) for i, _ in enumerate(x)]))

            plt.plot([_ for _ in range(0, len(accuracy))], accuracy, 'r--', label='Precision')
            plt.ylabel('Price differential')
            plt.xlabel('time')
            plt.grid(True)
            plt.title('Price differential = f(time)')
            plt.show()
    except:
        error('Not able to display data')

if __name__ == '__main__':
    main()
