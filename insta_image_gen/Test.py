from ImageGenerator import ImageGenerator
from PIL import Image
import os

class Test:
    def __init__(self) -> None:
        self.image = ImageGenerator("test_assets/image.png")
        self.save_path = "test_assets"

    def is_square(self):
        square_image_path = os.path.join(self.save_path, "square.png")
        self.image.make_square()
        self.image.save(square_image_path)

        square_image = Image.open(square_image_path)

        image_size = square_image.size
        assert image_size == (1000, 1000)
        print("Is Square Test Passed")

    def blur(self):
        self.image.create_background()
        print("Blur Test Passed")

    def crop_main_image(self):
        self.image.crop_main_image()
        print("Crop main image test passed!")


if __name__ == "__main__":
    t = Test()
    t.is_square()
    t.blur()
    t.crop_main_image()