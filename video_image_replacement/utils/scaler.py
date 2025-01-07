import math


def scale(dest: (int, int), comparasion: int, axis: str) -> (int, int):
    '''
    scales the given destination x and y coordinates on given axis to 
    fit comparasion value

    ex:
    scale(720, 480, 423, 'x')
    will return (423, 282)
    '''
    dest_width, dest_height, _ = dest
    w, h = None, None
    
    if axis == "x":
        w = dest_width
        h = dest_height
    else:
        w = dest_height
        h = dest_width
    
    scale = comparasion / w
    w *= scale
    h *= scale

    w = math.floor(w)
    h = math.floor(h)

    if axis == 'y':
        return (h, w)
    return (w, h)

def test_scale() -> None:
    assert scale((1920, 1080, 0), 1080, 'x') == (1080, 607)
    assert scale((1920, 1080, 0), 1080, 'y') == (1920, 1080)
    assert scale((1920, 1080, 0), 3840, 'x') == (3840, 2160)
    assert scale((1989, 564, 0), 216, 'y') == (761, 216)

if __name__ == "__main__":
    test_scale()