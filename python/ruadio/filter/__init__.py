from ..ruadio import filter as _filter

__doc__ = _filter.__doc__
__all__ = [
    'LinearInterpDelay',
    'SincInterpDelay',
]

LinearInterpDelay = _filter.LinearInterpDelay
SincInterpDelay = _filter.SincInterpDelay
