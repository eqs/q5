from .engine import *  #noqa F403
from .app import BaseApp, Vector

try:
    from ._version import version
    __version__ = version
except ImportError:
    pass

__author__ = 'eqs'
__all__ = [
    # Python
    'BaseApp',
    'Vector'
    # attrs
    'frame_count',
    'width',
    'height',
    'mouse_x',
    'mouse_y',
    'mouse_button',
    'key',
    # functions
    'run',
    'loop_forever',
    'no_loop',
    'loop_ntimes',
    'loop_wait',
    'title',
    'size',
    'full_screen',
    'push_matrix',
    'pop_matrix',
    'scale',
    'rotate',
    'translate',
    'fill',
    'no_fill',
    'stroke',
    'no_stroke',
    'stroke_weight',
    'background',
    'ellipse',
    'circle',
    'rect',
    'line',
    'arrow',
    'polygon',
    'polyline',
    'save_frame',
]
