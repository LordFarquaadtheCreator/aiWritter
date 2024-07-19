def main_process():
    from video_image_replace import video_image_replace
    from get_first_frame import get_frame_from_video
    import os
    from generate_replacement_frame import replacement_frame
    # GMT20240614-215723_Recording_640x360.mp4
    PWD = os.path.dirname(os.path.realpath(__file__))
    ROOT = '/Users/fahadfaruqi'
    INPUT_VIDEO_PATH = os.path.join(ROOT, "Downloads", input(
        "Enter file name of video in Downloads folder:\n"
    ))
    if not os.path.exists(INPUT_VIDEO_PATH):
        print("Error: File not found", INPUT_VIDEO_PATH)
        return main_process()

    FRAME_TO_GRAB = input("Enter time of frame to replace from video (in seconds):\n")
    if not FRAME_TO_GRAB.isdigit():
        print("Error: Invalid time")
        return main_process()

    OUTPUT_VIDEO_PATH_VIDEO = os.path.join(PWD, "assets/edited_video.mp4")  # hardcoded
    OUTPUT_VIDEO_PATH_FINAL = os.path.join(ROOT, "Downloads/processed_final.mp4")
    SHAPE, IMAGE_TO_DELETE = get_frame_from_video(INPUT_VIDEO_PATH, int(FRAME_TO_GRAB))

    USE_CYNTHIA = True
    if USE_CYNTHIA:
        REPLACEMENT_IMAGE = (
            os.path.join(PWD, "assets/cynthia.jpeg")  # hardcode if USE_CYNTHIA, future use only
        )

        print("The Default Photo of Cynthia will be used - target.png")

    print("Generating Target Frame")
    REPLACEMENT_FRAME = replacement_frame(PWD, SHAPE)

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
