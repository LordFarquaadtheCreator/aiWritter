def get_video_path(ROOT, EXIT_KEY) -> str:
    """
    gets video path from user
    checks for invalid path
    will recursively ask till exit key
    """
    import os
    from utils.exception import ExitQuery

    input_video_path_temp = input("Enter file name of video in Downloads folder:\n")
    INPUT_VIDEO_PATH = os.path.join(ROOT, "Downloads", input_video_path_temp)

    if input_video_path_temp == EXIT_KEY:
        raise ExitQuery
    if not os.path.exists(INPUT_VIDEO_PATH):
        print("Error: File not found", INPUT_VIDEO_PATH)
        return get_video_path(ROOT, EXIT_KEY)

    return INPUT_VIDEO_PATH
