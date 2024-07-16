def main_process():
    INPUT_VIDEO_PATH = "/Users/fahadfaruqi/Downloads/" + input(
        "Enter file name of video in Downloads folder:\n"
    )
    if not os.path.exists(INPUT_VIDEO_PATH):
        print("Error: File not found")
        return main_process()

    FRAME_TO_GRAB = input("Enter time of frame to replace from video (in seconds):\n")
    if not FRAME_TO_GRAB.isdigit():
        print("Error: Invalid time")
        return main_process()

    OUTPUT_VIDEO_PATH_VIDEO = "assets/edited_video.mp4"  # hardcoded
    OUTPUT_VIDEO_PATH_FINAL = "assets/processed_final.mp4"  # hardcoded
    SHAPE, IMAGE_TO_DELETE = get_frame_from_video(INPUT_VIDEO_PATH, int(FRAME_TO_GRAB))

    USE_CYNTHIA = True
    if USE_CYNTHIA:
        REPLACEMENT_IMAGE = (
            "assets/cynthia.jpeg"  # hardcode if USE_CYNTHIA, future use only
        )
        print("The Default Photo of Cynthia will be used - target.png")

    print("Generating Target Frame")
    REPLACEMENT_FRAME = replacement_frame(SHAPE)

    print("Please Wait as We Process the Video")
    main(
        IMAGE_TO_DELETE,
        REPLACEMENT_FRAME,
        INPUT_VIDEO_PATH,
        OUTPUT_VIDEO_PATH_VIDEO,
        OUTPUT_VIDEO_PATH_FINAL,
    )


if __name__ == "__main__":
    from video_image_replace import main
    from get_first_frame import get_frame_from_video
    import os
    from generate_replacement_frame import replacement_frame

    main_process()
