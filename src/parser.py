def parse(filename):
    with open(filename) as f:
        text = f.read()
    print(text)
    return None, None, None