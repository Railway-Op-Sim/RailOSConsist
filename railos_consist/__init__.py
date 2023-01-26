import PySimpleGUI as psg
import railostools.ttb.parsing.components as railos_ttb_comp
import railostools.exceptions as railos_exc
import railos_consist.data as railos_data
import pycountry
import typing
import sys
import pyperclip

psg.theme("Default1")


_countries: typing.Dict[str, str] = {i.alpha_2: i.name for i in pycountry.countries}


def _get_code(country: str) -> str:
    for key, value in _countries.items():
        if value == country:
            return key
    return ""


_country_list: typing.List[str] = [_countries[i] for i in railos_data.consists.keys()]

_ref_col = psg.Column(
    [
        [psg.Text("Reference")],
        [psg.Input(default_text="1A00", size=(10, 1), key="REF", enable_events=True)],
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
                values=list(range(0, 999)),
                initial_value=0,
                size=(10, 1),
                key="START_SPEED",
                enable_events=True,
                bind_return_key=True
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
                _country_list,
                enable_events=True,
                readonly=True,
                default_value=_country_list[0],
                tooltip="Country Code",
                key="COUNTRY_SELECT",
            )
        ],
    ]
)
_select_consist = psg.Column(
    [
        [psg.Text("Consist")],
        [
            psg.Combo(
                list(railos_data.consists[list(railos_data.consists.keys())[0]].keys()),
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
    [psg.Text(size=(70, 1), key="OUT_TEXT", text_color="black"), psg.Button("Copy", key="COPY")],
]

_window = psg.Window("RailOS TTB Header Generator", _app_layout)

_event_keys: typing.Set[str] = ["CONSIST_SELECT", "START_SPEED", "DESC", "REF", "COUNTRY_SELECT"]
_copyable_text: str = ""

while True:
    _event, _values = _window.read()
    if _event == psg.WIN_CLOSED or _event == "Exit":
        _window.close()
        sys.exit(0)
    if _event == "COUNTRY_SELECT":
        _window["CONSIST_SELECT"].update(
            values=list(
                railos_data.consists[_get_code(_values["COUNTRY_SELECT"])].keys()
            )
        )
    if _event in _event_keys:
        if any(not _values.get(i, None) and i != "START_SPEED" for i in _event_keys):
            continue
        _country_consists = railos_data.consists[_get_code(_values["COUNTRY_SELECT"])]
        _consist_template = _country_consists.headers[_values["CONSIST_SELECT"]]
        if (_start_speed := _values["START_SPEED"]):
            _max_speed: int = _country_consists.max_speeds[_values["CONSIST_SELECT"]]
            if _max_speed < _start_speed:
                _window["OUT_TEXT"].update(
                    value="Error: Start speed cannot be greater than maximum speed.",
                    text_color="red"
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
                text_color="red"
            )
            continue
        _window["OUT_TEXT"].update(
            value=_copyable_text,
            text_color="black"
        )
    if _event == "COPY" and _copyable_text:
        pyperclip.copy(_copyable_text)
