import cv2
import numpy as np
from PIL import Image
import imagehash
from moviepy.editor import VideoFileClip, AudioFileClip

# TODO: turn this into a CLI and combine with frame grabber, pushing all generated files to an untracked folder
# the replaced frame also needs to be fitted onto the canvas just a little bit better

IMAGE_TO_DELETE = "frame_to_delete.png"
REPLACEMENT_IMAGE = "../Downloads/IMG_1207.jpeg"
INPUT_VIDEO_PATH = "../Downloads/GMT20240614-215723_Recording_640x360.mp4"
OUTPUT_VIDEO_PATH_VIDEO = "processed_video.mp4"
OUTPUT_VIDEO_PATH_FINAL = "processed_final.mp4"

# Load the target and replacement images
target_image = cv2.imread(IMAGE_TO_DELETE)
replacement_image = cv2.imread(REPLACEMENT_IMAGE)

# Generate the hash for the target image
target_image_pil = Image.open(IMAGE_TO_DELETE)
target_hash = imagehash.phash(target_image_pil)

# Open the video file
cap = cv2.VideoCapture(INPUT_VIDEO_PATH)
fourcc = cv2.VideoWriter_fourcc(*"mp4v")
fps = cap.get(cv2.CAP_PROP_FPS)
width = int(cap.get(cv2.CAP_PROP_FRAME_WIDTH))
height = int(cap.get(cv2.CAP_PROP_FRAME_HEIGHT))

out = cv2.VideoWriter(OUTPUT_VIDEO_PATH_VIDEO, fourcc, fps, (width, height))

print("Starting To Replace!")
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

# Extract the audio from the original video
original_video = VideoFileClip(INPUT_VIDEO_PATH)
audio = original_video.audio
audio.write_audiofile("temp_audio.mp3")

# Combine the processed video with the extracted audio
processed_video = VideoFileClip(OUTPUT_VIDEO_PATH_VIDEO)
processed_video = processed_video.set_audio(AudioFileClip("temp_audio.mp3"))
processed_video.write_videofile(OUTPUT_VIDEO_PATH_FINAL, codec="libx264")

print("Processing complete! Final video saved as", OUTPUT_VIDEO_PATH_FINAL)
