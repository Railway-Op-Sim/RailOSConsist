import dataclasses
import pandas
import collections.abc
import typing


@dataclasses.dataclass
class ConsistLibrary(collections.abc.Mapping):
    country_code: str 
    data_sources: typing.List[str]
    separator: str = ";"
    _header_data: typing.Dict[str, typing.List[int]] = dataclasses.field(default_factory=dict)

    def __post_init__(self) -> None:
        pass

    @property
    def headers(self) -> typing.Dict[str, str]:
        return {
            key: f"{self.separator}".join(
                ["{reference}", "{description}", "{start_speed}"] +
                [str(i) for i in value]
            )
            for key, value in self._header_data.items()
        }

    @property
    def max_speeds(self) -> typing.Dict[str, int]:
        return {
            key: value[0] for key, value in self._header_data.items()
        }

    @property
    def data_frame(self) -> pandas.DataFrame:
        _headers: typing.List[str] = ("max_speed", "mass", "brake_force", "power")
        _df_dict: typing.Dict[str, typing.Any] = {
            "label": list(self._header_data.keys())
        }
        _df_dict |= {
            column: [v[i] for v in self._header_data.values()]
            for i, column in enumerate(_headers)
        }
        return pandas.DataFrame(_df_dict)
        
    def __getitem__(self, __key: str) -> str:
        return self.headers[__key]

    def __len__(self) -> int:
        return self.headers.__len__()
    
    def __iter__(self) -> typing.Iterator[str]:
        return self.headers.__iter__()

    def __str__(self) -> str:
        return self.data_frame.__str__()
