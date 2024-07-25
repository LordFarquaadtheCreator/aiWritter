class EndToEndTest:
    def __init__(self):
        self.frame_to_delete = None
        self.replacement_frame = None
        self.input_video_path = "mocks/video.mp4"
        self.output_process_video_path = "assets/TEST_processed_video.mp4"
        self.output_final_video_path = "assets/TEST_final.mp4"
        self.shape = ()

    def __mute__(self):
        import io
        import sys

        text_trap = io.StringIO()
        sys.stdout = text_trap

    def __cleanup__(self):
        import sys

        sys.stdout = sys.__stdout__

    def test_get_frame(self):
        from utils.get_first_frame import get_frame_from_video

        self.__mute__()

        try:
            self.shape, self.frame_to_delete = get_frame_from_video(
                "mocks/video.mp4", 0
            )
        except Exception as e:
            self.__cleanup__()
            raise e
        self.__cleanup__()

    def test_generate_replacement_frame(self):
        from utils.generate_replacement_frame import replacement_frame
        import os

        self.__mute__()

        try:
            self.replacement_frame = replacement_frame(os.getcwd(), self.shape)
        except Exception as e:
            self.__cleanup__()
            raise e
        self.__cleanup__()

    def test_process_video(self):
        from utils.video_image_replace import process_video

        self.__mute__()

        try:
            process_video(
                self.frame_to_delete,
                self.replacement_frame,
                self.input_video_path,
                self.output_process_video_path,
            )
        except Exception as e:
            self.__cleanup__()
            raise e
        self.__cleanup__()

    def test_save_video(self):
        from utils.save_video import save_video

        self.__mute__()

        try:
            save_video(
                self.input_video_path,
                self.output_process_video_path,
                self.output_final_video_path,
            )
        except Exception as e:
            self.__cleanup__()
            raise e
        self.__cleanup__()


if __name__ == "__main__":
    """Shallow Tests, assumes modules it tests has proper error handling"""
    myTest = EndToEndTest()
    num_pass = 0
    num_tests = len(
        [
            method
            for method in dir(EndToEndTest)
            if callable(getattr(EndToEndTest, method)) and not method.startswith("__")
        ]
    )

    try:
        myTest.test_get_frame()
        num_pass += 1
        print("Passed Get Frame Test")
    except Exception as e:
        print(f"Get Frame Test Failed: {e}")

    try:
        myTest.test_generate_replacement_frame()
        num_pass += 1
        print("Passed Generate Frame Test")
    except Exception as e:
        print(f"Generate Frame Test Failed: {e}")

    try:
        myTest.test_process_video()
        num_pass += 1
        print("Passed Process Video Test")
    except Exception as e:
        print(f"Process Video Test Failed: {e}")

    try:
        myTest.test_save_video()
        num_pass += 1
        print("Passed Save Video Test")
    except Exception as e:
        print(f"Save Video Test Failed: {e}")

    print(f"Passed {num_pass}/{num_tests}")

    if num_pass != num_tests:
        exit(1)
