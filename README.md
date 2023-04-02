# weather
weather is a configurable weather fetching application in Rust

# Installing
You can install it from source by running 
    
    >cargo install 
    
in the root directory of the repository, provided you have a Rust toolchain set up.
Alternatively, you can use release binaries from tagged releases of this repository

# Usage
Weather currently has support for WeatherAPI only

To use any of the services that are supported by Weather, you'll need to provide your own credentials on your local installation once. They will be saved to a configuration file in your %APPDATA% on Windows, XDG-compliant configuration directory on Linux or BSD, or /Application Support on MacOS

To provide Weather with credentials, run the following command:

    >weather configure <provider>

where provider can be any of the following: weather-api, ...
After that, you'll need to set a provider to be your default one by running the following command:

    >weather default <provider>

Now, to get a weather report, run the following:

    >weather get <location> [date]

where `date` is an optional argument, that allows for an hourly or daily offset like this:

    >weather get <location> f5H
    >weather get <location> h5D
Where `f` stands for forecast data and `h` stands for historical data. If you attempt to put anything else into this argument, it will default to a current weather status report.

This means, you can get historical or forecast data with varying degree of success at this moment.
Notice, that different weather providers have different capabilities and your request might fail if a provider or your level of API access do not allow you to obtain data too far in the past or the future.

You can view additional flags by accessing the help

    >weather -h
    >weather --help