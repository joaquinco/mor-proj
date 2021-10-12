import sys


def log_err(*args, **kwargs):
    sys.stderr.write(*args, **kwargs)
