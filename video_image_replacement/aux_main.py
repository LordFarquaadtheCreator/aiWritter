def main_process():
    import os
    from utils.video_image_replace import video_image_replace
    from utils.get_first_frame import get_frame_from_video
    from utils.generate_replacement_frame import replacement_frame
    from utils.exception import ExitQuery
    from inputs.get_path import get_video_path
    from inputs.get_time import get_time_stamp
    from pathlib import Path

    EXIT_KEY = "q"
    ROOT = Path.home()
    CURR_DIRR = os.path.dirname(os.path.realpath(__file__))

    try:
        INPUT_VIDEO_PATH = get_video_path(ROOT, EXIT_KEY)
        FRAME_TO_GRAB = get_time_stamp(EXIT_KEY)
    except ExitQuery:
        return

    OUTPUT_VIDEO_PATH_VIDEO = os.path.join(
        CURR_DIRR, "assets/edited_video.mp4"
    )  # hardcoded
    OUTPUT_VIDEO_PATH_FINAL = os.path.join(ROOT, "Downloads/processed_final.mp4")
    SHAPE, IMAGE_TO_DELETE = get_frame_from_video(INPUT_VIDEO_PATH, int(FRAME_TO_GRAB))

    USE_CYNTHIA = True
    if USE_CYNTHIA:
        REPLACEMENT_IMAGE = os.path.join(
            CURR_DIRR, "assets/cynthia.jpeg"
        )  # hardcode if USE_CYNTHIA, future use only

        print("The Default Photo of Cynthia will be used - target.png")

    print("Generating Target Frame")
    REPLACEMENT_FRAME = replacement_frame(CURR_DIRR, SHAPE)

    print("Please Wait as We Process the Video")
    video_image_replace(
        IMAGE_TO_DELETE,
        REPLACEMENT_FRAME,
        INPUT_VIDEO_PATH,
        OUTPUT_VIDEO_PATH_VIDEO,
        OUTPUT_VIDEO_PATH_FINAL,
    )


if __name__ == "__main__":
    main_process()
