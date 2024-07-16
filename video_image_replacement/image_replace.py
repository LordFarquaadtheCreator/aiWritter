def main_process():
    INPUT_VIDEO_PATH = input("Enter relative file path of video file:\n")
    if not os.path.exists(INPUT_VIDEO_PATH):
        print("Error: File not found")
        return main_process()

    FRAME_TO_GRAB = input("Enter time of frame to replace from video (in seconds):\n")
    if not FRAME_TO_GRAB.isdigit():
        print("Error: Invalid time")
        return main_process()

    OUTPUT_VIDEO_PATH_VIDEO = "edited_video.mp4"  # hardcoded
    OUTPUT_VIDEO_PATH_FINAL = "processed_final.mp4"  # hardcoded
    IMAGE_TO_DELETE = get_frame_from_video(INPUT_VIDEO_PATH, int(FRAME_TO_GRAB))

    USE_CYNTHIA = True
    if USE_CYNTHIA:
        REPLACEMENT_IMAGE = "target.png"  # hardcode if USE_CYNTHIA, future use only
        print("The Default Photo of Cynthia will be used - target.png")

    print("Please Wait as We Process the Video")

    main(
        IMAGE_TO_DELETE,
        INPUT_VIDEO_PATH,
        OUTPUT_VIDEO_PATH_VIDEO,
        OUTPUT_VIDEO_PATH_FINAL,
    )


if __name__ == "__main__":
    from video_image_replace import main
    from get_first_frame import get_frame_from_video
    import os

    main_process()
