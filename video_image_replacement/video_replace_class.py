class video_replace:
    def __init__(self, path: str, time: int):
        from get_first_frame import get_frame_from_video

        self.OUTPUT_VIDEO_PATH_VIDEO = "assets/edited_video.mp4"
        self.OUTPUT_VIDEO_PATH_FINAL = "../../Downloads/processed_final.mp4"
        self.video_path = path
        self.time_seconds = time
        self.shape, self.frame_to_delete = get_frame_from_video(
            str(self.path), int(self.time)
        )
        self.replacement_frame = None

    def set_time(self, time: int):
        try:
            self.time_seconds = int(time)
        except ValueError:
            print("Value must be integer or string representation of integer")

    def set_path(self, path: str):
        import os

        if os.path.exists(str(path)):
            self.video_path = str(path)
        else:
            raise OSError("Path not found")

    def gen_frame_to_delete(self, path: str = None, time: int = None):
        """
        grabs frame to delete from video based on time and path
        optionally you can overload them to replace the class's constructor values
        """
        from get_first_frame import get_frame_from_video

        time = self.time_seconds if time is None else time
        path = self.video_path if path is None else path

        self.shape, self.frame_to_delete = get_frame_from_video(path, time)
        return self.image_to_delete

    def generate_target_frame(self):
        """
        generates target frame from previously set time
        returns said frame and saves to scope of class
        """
        from generate_replacement_frame import replacement_frame

        self.replacement_frame = replacement_frame(self.shape)
        return self.replacement_frame

    def replace_frame_from_video(self, replace_frame=None, new_frame=None):
        """
        replaces all instances of `self.image_to_delete`
        optionally takes in replacement & new frame
        should the entire thing be here??? idk
        """
        from video_image_replace import replace_frame_from_video

        try:
            replace_frame_from_video(
                self.frame_to_delete,
                self.replacement_frame,
                self.video_path,
                self.OUTPUT_VIDEO_PATH_VIDEO,
                self.OUTPUT_VIDEO_PATH_FINAL,
            )
        except Exception as ex:
            print("Ruh Roh")
            print(ex)
