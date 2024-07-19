def main():
    import subprocess
    import os

    venv_path = os.path.join(os.getcwd(), "env")
    if not os.path.exists(venv_path):
        raise FileNotFoundError("Virtual environment not found at {}".format(venv_path))

    venv_python = os.path.join(venv_path, "bin", "python3")
    if not os.path.exists(venv_python):
        raise FileNotFoundError("Python executable not found at {}".format(venv_python))

    print("Using Python executable at:", venv_python)

    subprocess.check_call([venv_python, "aux_main.py"])


if __name__ == "__main__":
    main()
