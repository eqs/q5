from .engine import *  #noqa F403
from .draw import *  #noqa F403
from .app import BaseApp, Vector

try:
    from ._version import version
    __version__ = version
except ImportError:
    pass

__author__ = 'eqs'
__all__ = [BaseApp, Vector]
