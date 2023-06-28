"""
RailOSConsist GUI
=================

Definitions for the Graphical User Interface as seen by the user.

"""

__date__ = "2023-01-28"
__author__ = "Kristian Zarebski"
__license__ = "GPLv3"
__copyright__ = "Copyright 2023, Kristian Zarebski"

import PySimpleGUI as psg

import railos_consist.common as railosc_common
import railos_consist.data.library as railos_data


def setup_application() -> psg.Window:
    """Defines the initial user interface before callback assignment."""

    psg.theme("Default1")

    _ref_col = psg.Column(
        [
            [psg.Text("Reference")],
            [
                psg.Input(
                    default_text="1A00", size=(10, 1), key="REF", enable_events=True
                )
            ],
        ],
        expand_x=True,
    )
    _desc_col = psg.Column(
        [[psg.Text("Description")], [psg.Input(key="DESC", enable_events=True)]],
        expand_x=True,
    )
    _start_speed_col = psg.Column(
        [
            [psg.Text("Start Speed (km/h)")],
            [
                psg.Spin(
                    values=list(range(999)),
                    initial_value=0,
                    size=(10, 1),
                    key="START_SPEED",
                    enable_events=True,
                    bind_return_key=True,
                )
            ],
        ],
        expand_x=True,
    )
    _select_country = psg.Column(
        [
            [psg.Text("Country")],
            [
                psg.Combo(
                    [
                        value
                        for key, value in railosc_common.countries.items()
                        if key in railos_data.consist_library.countries
                    ],
                    enable_events=True,
                    readonly=True,
                    default_value=(
                        railosc_common.countries[
                            railos_data.consist_library.countries[0]
                        ]
                        if railos_data.consist_library.countries
                        else None
                    ),
                    tooltip="Country Code",
                    key="COUNTRY_SELECT",
                    size=(40, 1),
                )
            ],
        ]
    )
    _select_consist = psg.Column(
        [
            [psg.Text("Consist")],
            [
                psg.Combo(
                    railos_data.consist_library.consists(
                        railos_data.consist_library.countries[0]
                    )
                    if railos_data.consist_library.countries
                    else [],
                    enable_events=True,
                    readonly=True,
                    key="CONSIST_SELECT",
                )
            ],
        ]
    )

    _app_layout = [
        [_ref_col, _desc_col, _start_speed_col],
        [
            _select_country,
            _select_consist,
        ],
        [
            psg.Text(
                ""
                if railos_data.consist_library.countries
                else "Error: No data found.",
                size=(70, 1),
                key="OUT_TEXT",
                text_color="black"
                if railos_data.consist_library.countries
                else "red",
            ),
            psg.Button("Copy", key="COPY"),
        ],
    ]

    return psg.Window("RailOS TTB Header Generator", _app_layout)
