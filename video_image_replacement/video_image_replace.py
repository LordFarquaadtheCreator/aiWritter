import cv2
import numpy as np
from PIL import Image
import imagehash
from moviepy.editor import VideoFileClip, AudioFileClip
from save_video import save_video

IMAGE_TO_DELETE = "frame_to_delete.png"
REPLACEMENT_IMAGE = "target.png"
INPUT_VIDEO_PATH = "../Downloads/GMT20240614-215723_Recording_640x360.mp4"
OUTPUT_VIDEO_PATH_VIDEO = "edited_video.mp4"
OUTPUT_VIDEO_PATH_FINAL = "processed_final.mp4"

def process_video(target_hash):
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


if __name__ == "__main__":
    target_image = cv2.imread(IMAGE_TO_DELETE)
    replacement_image = cv2.imread(REPLACEMENT_IMAGE)

    target_image_pil = Image.open(IMAGE_TO_DELETE)
    target_hash = imagehash.phash(target_image_pil); del(target_image_pil)

    process_video(target_hash)
    out_path = save_video(INPUT_VIDEO_PATH, OUTPUT_VIDEO_PATH_VIDEO, OUTPUT_VIDEO_PATH_FINAL)
    print("Processing complete! Final video saved as", out_path)
