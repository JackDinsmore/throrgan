from mesher import Mesher
from parser import parse
from scipy.io.wavfile import write
import numpy as np
import matplotlib.pyplot as plt

CHUNK_LENGTH = 3
MIDDLE_C = 261.625565
SUBDIVIDE = 7*7#*5#*5#49
SAMPLE_RATE = 22050

def cauchy(x, mean, gamma=0.01):
    return 1/(1 + ((x - mean) / gamma)**2) + 0.5/(1 + ((x - mean-2) / gamma)**2)\
        + 0.25/(1 + ((x - mean-3/2) / gamma)**2) + 0.125/(1 + ((x - mean-4/3) / gamma)**2)
def gauss(x, mean, sigma=0.01):
    return np.exp(-(x - mean)**2 / 2/sigma**2) + 0.5*np.exp(-(x - mean)**2 / 2/sigma**2)\
        + 0.25*np.exp(-(x - mean)**2 / 2/sigma**2) + 0.125*np.exp(-(x - mean)**2 / 2/sigma**2)

def compile(filename):
    header, instruments, notes = parse(filename)
    bare = filename[:filename.rfind('.')]
    mesher = Mesher(header, instruments)

    octaves = np.linspace(0, 7, 7*32)
    div_times = np.linspace(0, CHUNK_LENGTH, SAMPLE_RATE*CHUNK_LENGTH // SUBDIVIDE)
    frequencies = MIDDLE_C * 2**(octaves-3)
    amplitudes = np.zeros((len(frequencies), len(div_times)))

    def want_octave(t):
        index = int(t / CHUNK_LENGTH * 24)
        if index == 0:
            return 3
        if index == 1:
            return 3-0.1663
        if index <= 8:
            return 3
        if index < 12:
            return None
        if index == 12:
            return 3-0.1663
        if index == 13:
            return 3-0.333
        if index == 14:
            return 3-0.4165
        if index == 15:
            return 3-0.583
        if index <= 19:
            return 3-0.666
        if index <= 23:
            return 3-0.583

    for i, t in enumerate(div_times):
        want = want_octave(t)
        if want is not None:
            amplitudes[:,i] += gauss(octaves, want)

    plt.pcolormesh(div_times, octaves, amplitudes)
    plt.figure()

    times = np.linspace(0, CHUNK_LENGTH, SAMPLE_RATE*CHUNK_LENGTH)
    sub_times = times.reshape(amplitudes.shape[1], -1)
    template_array = np.cos(np.outer(frequencies, times) * 2 * np.pi)
    template_array = template_array.reshape((template_array.shape[0], amplitudes.shape[1], -1))
    print(template_array.shape)

    data = np.zeros((template_array.shape[1]-1, template_array.shape[2]))
    for slice_index in range(amplitudes.shape[1]-1):
        data[slice_index, :] = np.einsum("i,ij->j", amplitudes[:,slice_index], template_array[:,slice_index,:])
        for freq_order in range(amplitudes.shape[0]):
            data[slice_index, :] += amplitudes[freq_order, slice_index] * template_array[freq_order,slice_index,:]
    
    data = data.reshape(-1)

    data *= 1 / np.max(data)

    plt.plot(data)
    plt.show()

    data *= np.iinfo(np.int16).max / max(-np.min(data), np.max(data))
    write(bare+'.wav', SAMPLE_RATE, data.astype(np.int16))