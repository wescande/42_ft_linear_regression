#!/usr/bin/python3
import sys
import signal
import argparse
import csv
from util import *

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
    parser.add_argument('--coeficient', '-c', action="store", dest="coef", type=int,
                        help='Force use of given coeficient')
    parser.add_argument('--ordinate', '-o', action="store", dest="ord", type=int,
                        help='Force use of given ordinate')
    parser.add_argument('--mileage', '-m', action="store", dest="mileage", type=int,
                        help='Value given for price calculation')
    return parser.parse_args()


def check_input():
    if options.coef is None or options.ord is None:
        value = get_gradient_csv()
        if options.ord is None:
            if 'b' in value:
                options.ord = value['b']
            else:
                options.ord = 0
        if options.coef is None:
            if 'a' in value:
                options.coef = value['a']
            else:
                options.coef = 0
    if options.mileage is None:
        while True:
            print('\033[35mPlease enter a mileage:\033[0m')
            try:
                choice = input('\033[35m \033[0m')
            except EOFError:
                error('EOF on input. Exit..')
                sys.exit(0)
            except (KeyboardInterrupt, SystemExit):
                raise
            except:
                error('Unknown error on input. Exit...')
                sys.exit(0)
            if choice.isdigit():
                options.mileage = int(choice)
                break
            error('{}: Not a valid value'.format(choice))
    return options.coef, options.ord, options.mileage


def main():
    """
        Main of prediction
    """
    signal.signal(signal.SIGINT, signal_handler)
    global options
    options = optparse()
    get_data_csv()
    a, b, x = check_input()
    try:
        price = estimated_price(a, x, b)
    except:
        error('No price could be calculated')
        sys.exit(1)
    if price:
        success('This car worth {} euros'.format(round(price, 2)))
    else:
        success('This car worth 0 euro')


if __name__ == '__main__':
    main()
