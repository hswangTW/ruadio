import unittest
import numpy as np
from ruadio.filter import LinearInterpDelay, SincInterpDelay

class TestLinearInterpDelay(unittest.TestCase):
    def _test_process_helper(self, dtype):
        delay = 3.3
        delay_filter = LinearInterpDelay(delay)

        x = [0] * 100 if dtype == list else np.zeros(100, dtype=dtype)
        x[0] = 1
        y = delay_filter.process(x)

        self.assertEqual(y.dtype, np.float32) # Always returns float32
        self.assertEqual(len(y), len(x))
        for n in [0, 1, 2] + list(range(5, 100)):
            self.assertAlmostEqual(y[n], 0.0)
        self.assertAlmostEqual(y[3], 0.7)
        self.assertAlmostEqual(y[4], 0.3)

    def test_process_float32(self):
        self._test_process_helper(np.float32)

    def test_process_float64(self):
        self._test_process_helper(np.float64)

    def test_process_int32(self):
        self._test_process_helper(np.int32)

    def test_process_int64(self):
        self._test_process_helper(np.int64)

    def test_process_list(self):
        self._test_process_helper(list)

    def test_process_invalid_list(self):
        delay = 3.3
        delay_filter = LinearInterpDelay(delay)
        with self.assertRaises(ValueError):
            delay_filter.process(['a', 'b', 'c', 'd', 'e'])

    def test_process_invalid_type(self):
        delay = 3.3
        delay_filter = LinearInterpDelay(delay)
        with self.assertRaises(TypeError):
            delay_filter.process(1)

if __name__ == '__main__':
    unittest.main()