from typing import List, Optional, Union
import numpy as np

class LinearInterpDelay:
    '''
    Delay filter based on linear interpolation.
    '''
    def __init__(self, delay: float) -> None:
        '''
        Delay filter based on linear interpolation.

        Parameters
        ----------
        delay : float
            The delay in samples, can be any positive real number.
        '''
        ...

    def process(self, input: Union[List[float], np.ndarray]) -> np.ndarray:
        '''
        Process the input signal.

        Parameters
        ----------
        input : Union[List[float], np.ndarray]
            The input signal, can be a list or a numpy array with 32/64-bit float or int as dtype.

        Returns
        -------
        np.ndarray
            The processed signal.
        '''
        ...

    def reset(self) -> None:
        '''
        Reset the state of the filter.
        '''
        ...

class SincInterpDelay:
    '''
    Delay filter based on sinc interpolation.
    '''
    def __init__(self, delay: float, sinc_half_width: Optional[int] = None, window_type: Optional[str] = None) -> None:
        '''
        Delay filter based on sinc interpolation.

        Parameters
        ----------
        delay : float
            The delay in samples, can be any positive real number.
        sinc_half_width : Optional[int]
            The half width of the sinc filter in samples. If not provided, a proper value is
            automatically chosen. Note that because the sinc filter also introduces a delay,
            `sinc_half_width` must not be greater than `delay + 0.5`.
        window_type : Optional[str]
            The window function to use for the sinc filter. The available window functions include
            'hamming' and 'hann', and the default is Hamming window.
        '''
        ...

    def process(self, input: Union[List[float], np.ndarray]) -> np.ndarray:
        '''
        Process the input signal.

        Parameters
        ----------
        input : Union[List[float], np.ndarray]
            The input signal, can be a list or a numpy array with 32/64-bit float or int as dtype.

        Returns
        -------
        np.ndarray
            The processed signal.
        '''
        ...

    def reset(self) -> None:
        '''
        Reset the state of the filter.
        '''
        ...
