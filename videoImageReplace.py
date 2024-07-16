import cv2
import numpy as np
from PIL import Image
import imagehash

# Load the specific image you want to detect and replace
target_image_path = "path_to_target_image.jpg"
replacement_image_path = "path_to_replacement_image.jpg"
target_image = cv2.imread(target_image_path)
replacement_image = cv2.imread(replacement_image_path)

# Generate the hash for the target image
target_image_pil = Image.open(target_image_path)
target_hash = imagehash.phash(target_image_pil)

# Open the video file
input_video_path = "path_to_input_video.mp4"
output_video_path = "path_to_output_video.mp4"
cap = cv2.VideoCapture(input_video_path)
fourcc = cv2.VideoWriter_fourcc(*"mp4v")
fps = cap.get(cv2.CAP_PROP_FPS)
width = int(cap.get(cv2.CAP_PROP_FRAME_WIDTH))
height = int(cap.get(cv2.CAP_PROP_FRAME_HEIGHT))

out = cv2.VideoWriter(output_video_path, fourcc, fps, (width, height))

while cap.isOpened():
    ret, frame = cap.read()
    if not ret:
        break

    # Convert the current frame to PIL image and generate hash
    frame_pil = Image.fromarray(cv2.cvtColor(frame, cv2.COLOR_BGR2RGB))
    frame_hash = imagehash.phash(frame_pil)

    # Compare the hash with the target image hash
    if frame_hash == target_hash:
        frame = cv2.resize(replacement_image, (width, height))

    # Write the frame to the output video
    out.write(frame)

# Release resources
cap.release()
out.release()
cv2.destroyAllWindows()
