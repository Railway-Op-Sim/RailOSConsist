import pytest
import pytest_cov.embed
import json
import pathlib
import random
import os.path
import typing
import multiprocessing

import railos_consist.data.library as railosc_library
import railos_consist.gui as railosc_gui

TEST_FILE: str = os.path.join(
    pathlib.Path(__file__).parents[1],
    "railos_consist",
    "data",
    "gb",
    "electric_multiple_units.json"
)

@pytest.fixture
def random_entry() -> typing.Tuple[str, typing.Dict[str, int]]:
    _data = json.load(open(TEST_FILE))
    _random_key = random.choice(list(_data.keys()))
    return _random_key, _data[_random_key]


@pytest.fixture(scope="module")
def consist_library() -> railosc_library.ConsistLibrary:
    return railosc_library.ConsistLibrary()


def test_consist_access(consist_library: railosc_library.ConsistLibrary, random_entry: typing.Tuple[str, typing.Dict[str, int]]) -> None:
    assert consist_library[random_entry[0]]["max_speed"] == random_entry[1]["max_speed"]
    assert len(consist_library)
    assert [i for i in consist_library]
    assert consist_library.max_speeds("gb")[random_entry[0]] == random_entry[1]["max_speed"]


def test_header_creation(consist_library: railosc_library.ConsistLibrary, random_entry: typing.Tuple[str, typing.Dict[str, int]]) -> None:
    assert consist_library.headers("gb")[random_entry[0]] == "{reference};{description};{start_speed};"+";".join(str(i) for i in random_entry[1].values())


def test_app_setup_() -> None:
    assert railosc_gui.setup_application()
