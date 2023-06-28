<p align="center">
<img
    style="display: block; 
           margin-left: auto;
           margin-right: auto;
           width: 30%;"
    src="https://raw.githubusercontent.com/Railway-Op-Sim/RailOSConsist/main/media/RailOSConsist.png" 
    alt="Our logo">
</img>
</p>

# Timetable Service Header Creator

_RailOSConsist_ is a small utility for quickly generating the first line of a service timetable entry in [Railway Operation Simulator](https://www.railwayoperationsimulator.com/) (RailOS) using existing consists. Currently the utility makes use of data assembled by Discord user _Mark "The Jinx"_ on the RailOS server for entries describing UK rolling stock with the plan to add entries for other countries at a later date.

## Creating Headers

The application is easy to use, just select a country from the dropdown and then a consist. Fill in the additional information and the header string will automatically be created. Use the _Copy_ button to copy the string to your clipboard.

When updating the start speed by typing values as opposed to adjusting with the spinbox, hit the `Enter` key to update the header.

![screenshot](https://raw.githubusercontent.com/Railway-Op-Sim/RailOSConsist/main/media/railosconsist_screenshot.png)

## Contributing

### Data

Country data is stored within the `railosconsist/data` directory, the information is kept in JSON files held in directories named in lower case from the ISO-2 country code for that country. For example the Class 139 Diesel Multiple Unit from the UK can be found in `railosconsist/data/gb/diesel_multiple_units.json`:

```json
{
  "Class 139": {
    "max_speed": 64,
    "mass": 12,
    "brake_force": 7,
    "power": 64
  },
  ...
}
```
