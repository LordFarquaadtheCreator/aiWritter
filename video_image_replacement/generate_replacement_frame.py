def replacement_frame(dim: (int, int)):
    """
    Generates replacement frame given nxm dimensions of frame to generate
    Defaults to use Cythnia Horner journalism photo
    IDEA: USE THIS TO POSSIBLY EVEN ADD THE LOGO?
    GOTTA MAKE IT GENERAL ...
    """
    import cv2
    from PIL import Image

    CYNTHIA_SCALE = 75
    # get cynthia's image
    cynthia = Image.open("assets/cynthia.jpeg")

    # resize her photo
    percentage_to_fill = (dim[1] / cynthia.size[1]) * (CYNTHIA_SCALE / 100)
    cynthia = cynthia.resize(
        (
            int(cynthia.size[0] * percentage_to_fill),
            int(cynthia.size[1] * percentage_to_fill),
        )
    )
    cynthia_center = [cynthia.size[0] // 2, cynthia.size[1] // 2]
    canvas_center = [dim[0] // 2, dim[1] // 2]
    adjusted_center = [
        canvas_center[0] - cynthia_center[0],
        canvas_center[1] - cynthia_center[1],
    ]

    # create blank canvas of DIM dimensions
    canvas = Image.new("RGB", dim, color=(28, 28, 28))  # color of zoom background
    # add cynthia's photo to middle
    canvas.paste(cynthia, adjusted_center)
    # return encoded image
    canvas.save("assets/target.png")
    return cv2.imread("assets/target.png")
