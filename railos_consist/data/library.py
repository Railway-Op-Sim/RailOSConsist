"""
RailOSConsist Library
=====================

Contains classes and objects for assembling all consist data into a
single accessible object which can be used by the main application.

"""

__date__ = "2023-01-28"
__author__ = "Kristian Zarebski"
__license__ = "GPLv3"
__copyright__ = "Copyright 2023, Kristian Zarebski"

import collections
import collections.abc
import dataclasses
import glob
import itertools
import json
import os
import pathlib
import typing

import railos_consist.common as railosc_common

DATA_DIRECTORY: str = os.path.join(pathlib.Path(__file__).parents[1], "data")


@dataclasses.dataclass
class ConsistLibrary(collections.abc.Mapping):
    """
    Consist library
    ===============

    Contains all consist data separated by country, this class assembles the
    template consist string which is then passed to the application for creating
    the header for a timetable service.
    """

    data_directory: str = DATA_DIRECTORY  # Where to search for data files
    separator: str = ";"  # Separator for data in header

    def __post_init__(self) -> None:
        """Post-initialisation which extracts data from directories."""
        self._data_keys: typing.Set[str] = {"max_speed", "power", "brake_force", "mass"}

        # Extra data from the data folder on a per country basis
        _data_directories: typing.List[str] = [
            os.path.join(DATA_DIRECTORY, direc)
            for direc in os.listdir(DATA_DIRECTORY)
            if os.path.isdir(os.path.join(DATA_DIRECTORY, direc))
            and direc in [c.lower() for c in railosc_common.countries.keys()]
        ]
        _data_files: typing.Dict[str, typing.List[str]] = {
            os.path.basename(direc): glob.glob(os.path.join(direc, "*.json"))
            for direc in _data_directories
        }
        self._header_data: typing.Dict[str, typing.Dict[str, typing.Dict[str, int]]] = {
            country_code: dict(
                collections.ChainMap(
                    *[json.load(open(file_name)) for file_name in file_list]
                )
            )  # type: ignore
            for country_code, file_list in _data_files.items()
        }

        # Sanity check to make sure all expected keys are present
        for country in self._header_data:
            for label, class_consist in self._header_data[country].items():
                for data_entry in self._data_keys:
                    if data_entry not in class_consist.keys():
                        raise AssertionError(
                            f"Expected entry '{data_entry}' "
                            f"in definition for consist '{label}'"
                        )

    @property
    def countries(self) -> typing.List[str]:
        return list(self._header_data.keys())

    def headers(self, country_code: str) -> typing.Dict[str, str]:
        """Returns all header templates for a given country code.

        Parameters
        ----------
        country_code : str
            ISO 2 country code

        Returns
        -------
        Dict[str, str]
            all consist entries for the given country
        """
        return {
            key: f"{self.separator}".join(
                ["{reference}", "{description}", "{start_speed}"]
                + [str(i) for i in data.values()]
            )
            for key, data in self._header_data[country_code].items()
        }

    def consists(self, country_code: str) -> typing.List[str]:
        """Returns all consists for the given country.

        Parameters
        ----------
        country_code : str
            ISO 2 country code

        Returns
        -------
        List[str]
            list of consists for the given country
        """
        return list(self._header_data[country_code].keys())

    def max_speeds(self, country_code: str) -> typing.Dict[str, int]:
        """Returns a dictionary of maximum speeds for a given country.

        Parameters
        ----------
        country_code : str
            ISO 2 country code

        Returns
        -------
        Dict[str, str]
            all consist maximum speed values for the given country
        """
        return {
            key: value["max_speed"]
            for key, value in self._header_data[country_code].items()
        }

    def __getitem__(self, __key: str) -> typing.Dict[str, int]:
        for country in self._header_data:
            if __key in self._header_data[country]:
                return self._header_data[country][__key]
        raise KeyError(f"Item '{__key}' was not found")

    def __len__(self) -> int:
        return sum([self._header_data[key].__len__() for key in self._header_data])

    def __iter__(self) -> typing.Iterator[str]:
        return itertools.chain(*self._header_data.values())


consist_library = ConsistLibrary()
