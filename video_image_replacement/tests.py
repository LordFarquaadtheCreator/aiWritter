class unit_tests:
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

        _, _ = get_frame_from_video("mocks/video.mp4", 0)
        _, _ = get_frame_from_video("mocks/video.mp4", -1)

        self.__cleanup__()

    def test_generate_replacement_frame(self):
        from utils.generate_replacement_frame import replacement_frame
        import os

        self.__mute__()

        _ = replacement_frame(os.getcwd(), (100, 100))

        self.__cleanup__()


if __name__ == "__main__":
    """Shallow Tests, assumes modules it tests has proper error handling"""
    myTest = unit_tests()
    num_pass = 0
    num_tests = len(
        [
            method
            for method in dir(unit_tests)
            if callable(getattr(unit_tests, method)) and not method.startswith("__")
        ]
    )

    try:
        myTest.test_get_frame()
        num_pass += 1
    except Exception as e:
        print(f"Get Frame Test Failed: {e}")

    try:
        myTest.test_generate_replacement_frame()
        num_pass += 1
    except Exception as e:
        print(f"Generate Frame Test Failed: {e}")

    print(f"Passed {num_pass}/{num_tests}")

    if num_pass != num_tests:
        exit(1)
