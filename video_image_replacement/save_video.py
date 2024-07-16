def save_video(old_video_path: str, new_video_path: str, output_file_name: str) -> str:
    from moviepy.editor import VideoFileClip, AudioFileClip
    try :
        old_video_path = VideoFileClip(old_video_path)
        new_video_path = VideoFileClip(new_video_path)
        # extract audio from old video
        audio: AudioFileClip = old_video.audio
        # save audio from old video to new video
        new_video.set_audio(audio)
        # write new video to disk
        new_video.write_videofile(output_file_name)

        return output_file_name
    except exception:
        print("Error occured saving video and audio")
        print(exception)
        print("Exiting ...")
        exit()