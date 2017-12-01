#!/usr/bin/env python3
import csv
import sys

def signal_handler(signal, frame):
    """
        Signal handler for a proper exit on Ctrl-C
    """
    print('\n\033[31mKeyboardInterrupt\033[0m')
    sys.exit(1)

def get_gradient_csv():
    try:
        with open('gradient.csv', 'r') as csvfile:
            content = csv.reader(csvfile, delimiter=',')
            line = {_[0]:float(_[1]) for _ in content}
    except:
        line = {'a':0, 'b':0}
    for _ in 'ab':
        if not _ in line:
            line[_] = 0
    return line

def get_data_csv():
    x = []
    y = []
    order = None
    try:
    # if True:
        with open('data.csv', 'r') as csvfile:
            content = csv.reader(csvfile, delimiter=',')
            for _ in content:
                if order is None:
                    if _[0] != 'km' or _[1] != 'price':
                        print('wrong csv format')
                        sys.exit(1)
                    else:
                        order = True
                if _[0].isdigit():
                    x.append(int(_[0]))
                    y.append(int(_[1]))
    except:
        print('Unable to analyze data.csv')
        sys.exit(1)
    global mile_lim
    global mile_delta
    mile_lim = [min(x), max(x)]
    mile_delta = mile_lim[1] - mile_lim[0]
    if not mile_delta:
        mile_delta = 1
    global price_lim
    price_lim = [min(y), max(y)]
    return x, y

def raw_estimated_price(a, x, b):
    return a * x + b

def estimated_price(a, x, b):
    price_ranged = raw_estimated_price(a, (x - mile_lim[0]) / mile_delta, b)
    return price_ranged * (price_lim[1] - price_lim[0]) + price_lim[0]
