from pathlib import Path
import shutil
import os

task_selected = None


def get_score(file_path: Path):
    score = 2**64
    if not file_path.exists():
        return 2**64
    try:
        score = int(file_path.open().read())
    except:
        return 2**64
    return score


max_score = 2**64

for x in Path('task_output').iterdir():
    score_file_path = Path(x.as_posix() + "/score")
    output_file_path = Path(x.as_posix() + "/output")

    if not x.is_dir() or not output_file_path.exists():
        continue

    score = get_score(score_file_path)
    if score < max_score:
        task_selected = output_file_path
        max_score = score


shutil.copy(task_selected, 'task.sol')

puzzle_selected = None
for x in Path('puzzle_output').iterdir():
    if puzzle_selected is None:
        puzzle_selected = x

shutil.copy(puzzle_selected, 'puzzle.desc')
