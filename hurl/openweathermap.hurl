GET https://api.openweathermap.org/geo/1.0/direct
[QueryStringParams]
q:Zaporizhzhia:
limit:5
appid:{{OPENWEATHER_API_KEY}}
units:metric

HTTP 200
[Captures]
lat: jsonpath "$[0]['lat']"
lon: jsonpath "$[0]['lon']"

GET https://api.openweathermap.org/data/2.5/weather
[QueryStringParams]
lat:{{lat}}
lon:{{lon}}
appid:{{OPENWEATHER_API_KEY}}

HTTP 200

GET https://api.openweathermap.org/data/2.5/forecast
[QueryStringParams]
lat:{{lat}}
lon:{{lon}}
appid:{{OPENWEATHER_API_KEY}}

HTTP 200
