if __name__ == "__main__":
    '''
    class based system
    the image is composed of four layers
    background -> blurred
    text
    background of text -> adjusted to complement yet stand out form background
    main image - cropped
    '''
    file_path = "some path, input maybe?"
    image = ImageGenerator(file_path)
    
    try:
        image.create_background() # will save the background to some dir
        image.create_text()
        image.create_text_background()
        image.crop_main_image()

        image.compose()
        print("Image Successfully Created, saved to downloads folder")
        exit(0)
    except Exception as e:
        print(e)
        exit(1)