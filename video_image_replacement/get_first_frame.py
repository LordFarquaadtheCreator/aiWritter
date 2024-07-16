def get_frame_from_video(VIDEO_PATH: str, TIME: int):
    import cv2
    from PIL import Image
    import imagehash

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
        frame = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
        frame_PIL = Image.fromarray(frame)
        frame_hash = imagehash.phash(frame_PIL)
        cap.release()

        return frame_hash
    else:
        print("Error: Could not read frame")
        cap.release()
        exit()
