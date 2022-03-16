import numpy as np

class Instrument:
    def __init__(self):
        return NotImplemented

    def profile(self, octave, mean):
        return NotImplemented

    def amp(self, t, true_dur):
        return NotImplemented


class Gaussian(Instrument):
    def __init__(self, bpm):
        self.pre_time = 0.02 * bpm / 60
        self.post_time = 0.05 * bpm / 60
        self.sigma = 0.01
    
    def profile(self, x, mean):
        return np.exp(-(x - mean)**2 / 2/self.sigma**2)\
            + 0.5 * np.exp(-(x - mean - 1)**2 / 2/self.sigma**2)\
            + 0.25 * np.exp(-(x - mean - 3/2)**2 / 2/self.sigma**2)\
            + 0.125 * np.exp(-(x - mean - 2)**2 / 2/self.sigma**2)

    def amp(self, t, true_dir):
        if t < 0:
            return (self.pre_time + t) / self.pre_time
        if t > true_dir:
            return (self.post_time - t + true_dir) / self.post_time
        return 1


class Cauchy(Instrument):
    def __init__(self):
        self.pre_time = 0.1
        self.post_time = 0.1
        self.gamma = 0.01
    
    def profile(self, x, mean):
        return 1/(1 + ((x - mean) / self.gamma)**2) + 0.5/(1 + ((x - mean-2) / self.gamma)**2)\
            + 0.25/(1 + ((x - mean-3/2) / self.gamma)**2) + 0.125/(1 + ((x - mean-4/3) / self.gamma)**2)

    def amp(self, t, true_dir):
        if t < 0:
            return (self.pre_time + t) / self.pre_time
        if t > true_dir:
            return (self.post_time - t + true_dir) / self.post_time
        return 1


class Note:
    def __init__(self, instruments, instrument_index, octave, start, dur):
        self.inst = instrument_index
        self.oct = octave
        self.start = start - instruments[instrument_index].pre_time
        self.full_start = start
        self.full_dur = dur
        self.dur = dur + instruments[instrument_index].pre_time + instruments[instrument_index].post_time

    def check(self, t):
        return (self.start < t and t < self.start + self.dur)