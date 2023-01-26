import pandas
import os.path
import glob
import railos_consist.data.library as railos_data


class UKConsists(railos_data.ConsistLibrary):
    def __post_init__(self) -> None:
        for source in self.data_sources:
            _dataframe: pandas.DataFrame = pandas.read_csv(
                source, header=None, skiprows=[0]
            )

            for i in range(len(_dataframe)):
                self._header_data[_dataframe[0].iloc[i]] = [
                    int(_dataframe[4].iloc[i]),
                    int(_dataframe[6].iloc[i]),
                    int(_dataframe[9].iloc[i]),
                    int(_dataframe[11].iloc[i]),
                ]


uk_consists = UKConsists("GB", glob.glob(os.path.join(os.path.dirname(__file__), "*.csv")))
