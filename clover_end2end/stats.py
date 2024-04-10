import glob
import os


def get_stats(dataset_dir, categories):
    filenames = []
    for category in categories:
        print(category, "\n")
        dirpath = os.path.join(dataset_dir, category, "*")
        filenames.append([name.strip("/").split("/")[-1] for name in glob.glob(dirpath) if name.endswith("dfy")])
        print(filenames[-1], "\n")
    return filenames


if __name__ == "__main__":
    # dataset_dir = "../dataset/CloverBench"
    dataset_dir = "../dataset/MBPP-DFY-50-legal"
    categories = ["gpt4_correct_and_anno_sound", "gpt4_correct_not_anno_sound",
                  "gpt4_incorrect_and_anno_sound", "gpt4_incorrect_not_anno_sound"]

    filenames = get_stats(dataset_dir, categories)
    for i, category in enumerate(categories):
        print(category, len(filenames[i]))

   
