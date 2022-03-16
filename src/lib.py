from mesher import Mesher
from parser import parse
from scipy.io.wavfile import write
import numpy as np
import matplotlib.pyplot as plt

CHUNK_LENGTH = 3.1
MIDDLE_C = 261.625565
SUBDIVIDE = 7*7#*5#*5#49
SAMPLE_RATE = 22050

def compile(filename):
    header, instruments, notes = parse(filename)
    bare = filename[:filename.rfind('.')]
    mesher = Mesher(header, instruments)

    octaves = np.linspace(0, 7, 7*32)
    div_times = np.linspace(0, CHUNK_LENGTH * header.bpm / 60, int(SAMPLE_RATE*CHUNK_LENGTH / SUBDIVIDE))
    frequencies = MIDDLE_C * 2**(octaves-3)
    amplitudes = np.zeros((len(frequencies), len(div_times)))

    on_notes = []
    for i, t in enumerate(div_times):
        # Turn on notes
        while len(notes) > 0 and notes[0].check(t):
            on_notes.append(notes[0])
            del notes[0]
        
        # Turn off notes
        note_index = 0
        while note_index < len(on_notes):
            if not on_notes[note_index].check(t):
                del on_notes[note_index]
            else:
                note_index += 1

        for n in on_notes:
            amplitudes[:,i] += instruments[n.inst].amp(t-n.full_start, n.full_dur) * instruments[n.inst].profile(octaves, n.oct)

    c = plt.pcolormesh(div_times, octaves, amplitudes)
    plt.colorbar(c)
    plt.figure()

    times = np.linspace(0, CHUNK_LENGTH, int(SAMPLE_RATE*CHUNK_LENGTH))
    template_array = np.cos(np.outer(frequencies, times) * 2 * np.pi)
    template_array = template_array.reshape((template_array.shape[0], amplitudes.shape[1], -1))

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