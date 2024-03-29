import requests
import random
from pathlib import Path

DEPTH = 3
FOLDERS = 5
FILES = 5

word_site = "https://www.mit.edu/~ecprice/wordlist.10000"
response = requests.get(word_site)
WORDS = response.content.decode().split("\n")


INIT_PATH = "./testdir"
Path(INIT_PATH).mkdir(parents=True, exist_ok=False)


FILENAME = "testcase"
FOLDERNAME = "testdir_"
PATH_QUEUE = ["./testdir"]

def create_tests(deep, path):
    
    
    for i in range(FILES):
        content = [random.choice(WORDS) for i in range(100)]

        with Path('{}/{}{}.txt'.format(path, FILENAME, i)).open("w") as f:
            f.write("\n".join(content))
    
    if deep != DEPTH:
        for i in range(FOLDERS):
            new_path = '{}/{}{}'.format(path, FOLDERNAME,i)
            Path(new_path).mkdir(parents=True, exist_ok=False)
            create_tests(deep + 1, new_path)
    
create_tests(1, INIT_PATH)