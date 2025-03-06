from datetime import datetime
from random import random
import subprocess

vec_string = [str(random()) for _ in range(512)]
start = datetime.now()
proc = subprocess.Popen(
    ["cargo", "run", "--release", "./words_subset.data", "512"] + vec_string
)
end = datetime.now()
print(end-start)