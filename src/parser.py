from instrument import Gaussian, Note

class Header:
    def __init__(self):
        self.bpm = None

def parse(filename):
    with open(filename) as f:
        lines = f.readlines()

    notes = []
    header = Header()
    mode = 0
    for line in lines:
        if "header" in line:
            mode = 1
            continue
        if "instruments" in line:
            mode = 2
            continue
        if "notes" in line:
            mode = 3
            continue
        if mode == 0:
            continue
        elif mode == 1:
            header.bpm = float(line)
        elif mode == 2:
            instruments = [Gaussian(header.bpm)]
        elif mode == 3:
            inst_index, octave, start, dur = line.split(', ')
            notes.append(Note(instruments, int(inst_index), float(octave), float(start), float(dur)))

    return header, instruments, notes