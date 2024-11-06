from ImageGenerator import ImageGenerator
from PIL import Image
import os

class Test:
    def __init__(self) -> None:
        self.image = ImageGenerator()
        self.save_path = "test_assets"

    def is_square(self):
        square_image_path = os.path.join(self.save_path, "square.png")
        self.image.make_square()
        self.image.save(square_image_path)

        square_image = Image(square_image_path)

        image_size = square_image.size
        assert image_size == (1000, 1000)