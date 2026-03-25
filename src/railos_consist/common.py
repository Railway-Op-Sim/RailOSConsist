"""
Common
======

Contains functions used by the module as a whole.
"""

import typing

import pycountry

countries: typing.Dict[str, str] = {
    i.alpha_2.lower(): i.name for i in pycountry.countries
}
