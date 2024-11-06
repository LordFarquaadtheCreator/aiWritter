from PIL import Image, ImageDraw
import os
class ImageGenerator:
    def __init__(self, image_path: str) -> None:
        self.curr_dirr = os.path.join(os.getcwd(), "aiWritter/insta_image_gen")
        self.logo_path = os.path.join(self.curr_dirr, "test_assets/image.png")
        if not os.path.exists(self.logo_path):
            raise Exception("Logo missing (name 'logo') from test_assets directory")
        if not os.path.exists(image_path):
            raise Exception("Cannot find image")
        
        self.image = Image.open(image_path)
        self.logo = Image.open(self.logo_path)
        self.background_image = None
        self.text_background = None
        self.text = None
        self.main_image = None
        self.dimensions = (1000, 1000)

    def make_square(self):
        '''
        image could be of any size -> we want to make it square
        take the shorter edge and scale it to fit from there
        crop it at dimension size
        '''
        width, height = self.image.size # assumption
        scale_factor = 1

        if width > height:
            scale_factor = self.dimensions[0] / height
        if height < width:
            scale_factor = self.dimensions[0] / width
        new_height = (width * scale_factor, height * scale_factor)
        
        scaled_up_image = self.image.resize(new_height)
        self.image = scaled_up_image.resize(self.dimensions)


    def create_background(self):
        pass
    def create_text(self):
        pass
    def create_text_background(self):
        pass
    def crop_main_image(self):
        pass
    def add_logo(self):
        pass
    def save(self, path: str = "~/Downloads/insta.png"):
        self.image.save(path)


if __name__ == "__main__":
    image_path = os.path.join(os.curdir(), "test_assets/image.png")
    c = ImageGenerator(image_path)