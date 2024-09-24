def sub(x, a, b):
    for a, b in zip(a, b):
        x = x.replace(a, b)
    return x

def matches(a, b):
    m = {}
    for a, b in zip(a, b):
        if a in m:
            if m[a] != b:
                return False
        else:
            m[a] = b
