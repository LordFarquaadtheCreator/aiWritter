import cv2
from PIL import Image
import imagehash
from save_video import save_video

REPLACEMENT_IMAGE = "target.png"


def process_video(
    target_hash, replacement_image, OUTPUT_VIDEO_PATH_VIDEO, input_video_path
):
    # Open the video file
    cap = cv2.VideoCapture(input_video_path)
    fourcc = cv2.VideoWriter_fourcc(*"mp4v")
    fps = cap.get(cv2.CAP_PROP_FPS)
    width = int(cap.get(cv2.CAP_PROP_FRAME_WIDTH))
    height = int(cap.get(cv2.CAP_PROP_FRAME_HEIGHT))

    out = cv2.VideoWriter(OUTPUT_VIDEO_PATH_VIDEO, fourcc, fps, (width, height))

    print("Parsing Over Video")
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


def main(
    target_image_hashed,
    INPUT_VIDEO_PATH,
    OUTPUT_VIDEO_PATH_VIDEO,
    OUTPUT_VIDEO_PATH_FINAL,
):
    replacement_image = cv2.imread(REPLACEMENT_IMAGE)

    process_video(
        target_image_hashed,
        replacement_image,
        OUTPUT_VIDEO_PATH_VIDEO,
        INPUT_VIDEO_PATH,
    )
    out_path = save_video(
        INPUT_VIDEO_PATH, OUTPUT_VIDEO_PATH_VIDEO, OUTPUT_VIDEO_PATH_FINAL
    )
    print("Processing complete! Final video saved as", out_path)
