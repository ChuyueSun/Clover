import os
import typing

import pydantic


class Specification(pydantic.BaseModel):
    method_signature: str
    preconditions: str
    postconditions: str


class Task(pydantic.BaseModel):
    code: str
    specification: Specification
    task_description: str
    task_id: str


class Dataset(pydantic.RootModel):
    root: typing.Dict[str, Task]

    def __iter__(self):
        return iter(self.root)

    def __getitem__(self, item):
        return self.root[item]

    def __setitem__(self, key, value):
        self.root[key] = value


def load_dataset() -> Dataset:
    dataset_file = os.path.join(
        os.path.dirname(os.path.abspath(__file__)),
        "../dataset/MBPP-DFY-50/mbpp-dfy-50-examples-db.json")
    with open(dataset_file, "r") as f:
        json_str = f.read()
    return Dataset.model_validate_json(json_str)


if __name__ == "__main__":
    dataset = Dataset({})
    print(dataset.model_dump_json(indent=2))
    dataset = load_dataset()
    print(dataset.model_dump_json(indent=2))
