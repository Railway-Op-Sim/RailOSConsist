"""
RailOSConsist Application
=========================

Main file for the RailOSConsist Utility written in PySimpleGUI.

Uses a globular function to assemble a consist library from JSON files with an assumed
key structure. Exceptions are caught to be displayed as red error messages to the user.

Note: The assembly of the application is held in a function to satisfy PyInstaller
(else it tries to launch the GUI during a build which causes it to hang).
"""

__date__ = "2023-01-28"
__author__ = "Kristian Zarebski"
__license__ = "GPLv3"
__copyright__ = "Copyright 2023, Kristian Zarebski"

import datetime
import os.path
import sys
import typing

import pyperclip
import PySimpleGUI as psg
import railostools.exceptions as railos_exc
import railostools.ttb.parsing.components as railos_ttb_comp

import railos_consist.common as railosc_common
import railos_consist.data.library as railos_data
import railos_consist.gui as railosc_gui

APP_LOCATION: str = os.path.dirname(__file__)


def launch_application() -> None:
    """Launch the Application assigning the callbacks to the GUI."""
    _window: psg.Window = railosc_gui.setup_application()

    _event_keys: typing.Set[str] = {
        "CONSIST_SELECT",
        "START_SPEED",
        "DESC",
        "REF",
        "COUNTRY_SELECT",
    }
    _copyable_text: str = ""

    def _get_code(country: str) -> str:
        for key, value in railosc_common.countries.items():
            if value == country:
                return key
        return ""

    while True:
        _event, _values = _window.read()
        if _event in [psg.WIN_CLOSED, "Exit"]:
            _window.close()
            sys.exit(0)
        try:
            if _event == "COUNTRY_SELECT":
                _window["CONSIST_SELECT"].update(
                    values=list(
                        railos_data.consist_library.consists(
                            _get_code(_values["COUNTRY_SELECT"])
                        )
                    )
                )
            if _event in _event_keys:
                if any(
                    not _values.get(i, None) and i != "START_SPEED" for i in _event_keys
                ):
                    continue
                _country: str = _get_code(_values["COUNTRY_SELECT"])
                _headers: typing.Dict[str, str] = railos_data.consist_library.headers(
                    _country
                )
                _max_speeds: typing.Dict[
                    str, int
                ] = railos_data.consist_library.max_speeds(_country)
                _consist_template = _headers[_values["CONSIST_SELECT"]]
                if _start_speed := _values["START_SPEED"]:
                    _max_speed: int = _max_speeds[_values["CONSIST_SELECT"]]
                    if _max_speed < _start_speed:
                        _window["OUT_TEXT"].update(
                            value="Error: Start speed cannot be greater than maximum speed.",
                            text_color="red",
                        )
                        continue
                _copyable_text = _consist_template.format(
                    reference=_values["REF"],
                    description=_values["DESC"],
                    start_speed=_values["START_SPEED"],
                )
                try:
                    railos_ttb_comp.parse_header(_copyable_text)
                except railos_exc.ParsingError:
                    _window["OUT_TEXT"].update(
                        value="Error: Validation of header string failed.",
                        text_color="red",
                    )
                    continue
                _window["OUT_TEXT"].update(value=_copyable_text, text_color="black")
            if _event == "COPY" and _copyable_text:
                pyperclip.copy(_copyable_text)
        except (KeyError, AssertionError) as e:
            _out_file_name: str = (
                f"error_{datetime.datetime.now().strftime('%Y%M%d%H%S')}.log"
            )
            _window["OUT_TEXT"].update(
                value=f"Error: See {_out_file_name}.", text_color="red"
            )
            with open(os.path.join(APP_LOCATION, _out_file_name), "w") as out_f:
                out_f.write(e.args[0])


if __name__ in "__main__":
    launch_application()
