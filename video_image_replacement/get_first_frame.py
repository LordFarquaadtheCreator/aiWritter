import cv2

VIDEO_PATH = "../Downloads/GMT20240614-215723_Recording_640x360.mp4"
TIME = 2 # seconds
OUTPUT_FILE_NAME = "frame_to_delete.png"
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
    output_image_path = OUTPUT_FILE_NAME
    cv2.imwrite(output_image_path, frame)
    print(f"Frame at {TIME} seconds saved as {output_image_path}")
else:
    print("Error: Could not read frame.")

# Release the video capture object
cap.release()
