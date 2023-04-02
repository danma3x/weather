GET http://api.weatherapi.com/v1/current.json
[QueryStringParams]
key: {{WEATHERAPI_API_KEY}}
q: Zaporizhzhya

HTTP 200

GET http://api.weatherapi.com/v1/forecast.json
[QueryStringParams]
key: {{WEATHERAPI_API_KEY}}
q: Zaporizhzhya
days: 3
aqi: no
alerts: no

HTTP 200

GET http://api.weatherapi.com/v1/history.json
[QueryStringParams]
key: {{WEATHERAPI_API_KEY}}
q: Zaporizhzhya
dt: 2023-03-01

HTTP 200