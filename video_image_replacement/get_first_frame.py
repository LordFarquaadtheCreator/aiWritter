def get_frame_from_video(VIDEO_PATH: str, TIME: int) -> cv2:
    import cv2

    cap = cv2.VideoCapture(VIDEO_PATH)

    if not cap.isOpened():
        print("Error: Could not open video.")
        exit()

    FPS = cap.get(cv2.CAP_PROP_FPS)

    # Calculate the frame number corresponding to the timestamp
    frame_number = int(TIME * FPS)

    # Set the video position to the frame number
    cap.set(cv2.CAP_PROP_POS_FRAMES, frame_number)

    # Read the frame at the specified timestamp
    ret, frame = cap.read()

    if ret:
        # Save the frame as an image file
        cap.release()
        return frame
    else:
        print("Error: Could not read frame")
        cap.release()
        exit()
