from moviepy.editor import VideoFileClip, AudioFileClip
def save_video(new_video: VideoFileClip, old_video: VideoFileClip, output_file_name: str) -> str:
    try :
        # extract audio from old video
        audio: AudioFileClip = old_video.audio
        # save audio from old video to new video
        new_video.set_audio(audio)
        # write new video to disk
        new_video.write_videofile(output_file_name)
        # return path
        return output_file_name
    except exception:
        print("Error occured saving video and audio")
        print(exception)
        print("Exiting ...")
        exit()