def get_time_stamp(EXIT_KEY) -> int:
    """
    gets time stamp of frame to grab from user
    checks for floats and strings
    recursively asks till exit key is pressed
    """
    from utils.exception import ExitQuery

    FRAME_TO_GRAB = input("Enter time of frame to replace from video (sec):\n")
    if FRAME_TO_GRAB == EXIT_KEY:
        raise ExitQuery
    if not FRAME_TO_GRAB.isdigit():
        print("Error: input must be an integer")
        return get_time_stamp(EXIT_KEY)

    return int(FRAME_TO_GRAB)
