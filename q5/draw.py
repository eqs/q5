import numpy as np
from .engine import (
    polygon_list, polygon_ndarray, polyline_list, polyline_ndarray
)

__all__ = ["polygon", "polyline"]


def polygon(pts):
    if isinstance(pts, np.ndarray):
        polygon_ndarray(pts)
    elif isinstance(pts, list):
        polygon_list(pts)
    else:
        raise TypeError('Invalid type is detected: {}'.format(type(pts)))


def polyline(pts):
    if isinstance(pts, np.ndarray):
        polyline_ndarray(pts)
    elif isinstance(pts, list):
        polyline_list(pts)
    else:
        raise TypeError('Invalid type is detected: {}'.format(type(pts)))
