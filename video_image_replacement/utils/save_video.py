def save_video(old_video_path: str, new_video_path: str, output_file_name: str) -> str:
    from moviepy.editor import VideoFileClip

    old_video = VideoFileClip(old_video_path)
    new_video = VideoFileClip(new_video_path)
    # Extract audio from old video
    audio = old_video.audio
    # Combine new video with audio from old video
    final_video = new_video.set_audio(audio)
    # Write the final video (with audio) to disk
    final_video.write_videofile(output_file_name, audio_codec="aac")

    return output_file_name
