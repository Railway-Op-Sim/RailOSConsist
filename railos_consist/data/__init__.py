import typing

import railos_consist.data.library as railos_data
import railos_consist.data.uk as uk_data

consists: typing.Dict[str, railos_data.ConsistLibrary] = {
    l.country_code: l for l in (uk_data.uk_consists,)
}
