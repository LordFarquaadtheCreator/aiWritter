class unit_tests:
    def __init__(self):
        import io
        import sys

        # Mute Outputs
        text_trap = io.StringIO()
        sys.stdout = text_trap

    def __cleanup__(self):
        import io
        import sys

        sys.stdout = sys.__stdout__

    def test_get_frame(self):
        from utils.get_first_frame import get_frame_from_video

        _, _ = get_frame_from_video("mocks/video.mp4", 0)
        _, _ = get_frame_from_video("mocks/video.mp4", -1)

        self.__cleanup__()


if __name__ == "__main__":
    myTest = unit_tests()
    num_pass = 0
    num_tests = len(
        [
            method
            for method in dir(unit_tests)
            if callable(getattr(unit_tests, method)) 
            and not method.startswith("__")
        ]
    )

    try:
        myTest.test_get_frame()
        num_pass += 1
    except Exception as e:
        print(f"Get Frame Test Failed: {e}")


    print(f"Passed {num_pass}/{num_tests}")
