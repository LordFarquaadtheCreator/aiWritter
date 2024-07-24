class unit_tests:
    def test_first_frame(self):
        from get_first_frame import get_frame_from_video
        get_frame_from_video("video.mp4", 0)
        return True


if __name__ == "__main__":
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
        myTest.test_first_frame()
        num_pass += 1
    except Exception as e:
        print(f"Get Frame Test Failed: {e}")

    print(f"Passed {num_pass}/{num_tests}")