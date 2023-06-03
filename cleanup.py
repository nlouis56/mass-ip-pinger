import os

if __name__ == "__main__":
    executing_path = os.path.dirname(os.path.realpath(__file__))
    folder = os.path.join(executing_path, "cut_list")
    rawinput = input("Enter the path to the cut list folder:  (default: {})".format(folder))
    if rawinput != "":
        folder = rawinput
    print("Starting...")
    for filename in os.listdir(folder):
        if filename != ".gitkeep":
            os.remove(os.path.join(folder, filename))