GET http://dataservice.accuweather.com/locations/v1/search
[QueryStringParams]
apikey: {{ACCUWEATHER_API_KEY}}
q: Zaporizhzhia

HTTP 200
[Captures]
location_key: jsonpath "$[0]['Key']"


GET http://dataservice.accuweather.com/currentconditions/v1/{{location_key}}
[QueryStringParams]
apikey: {{ACCUWEATHER_API_KEY}}

HTTP 200

#hourly steps 1 - 12 - 120 - 24 - 72

GET http://dataservice.accuweather.com/forecasts/v1/hourly/12hour/{{location_key}}
[QueryStringParams]
apikey: {{ACCUWEATHER_API_KEY}}
metric: true

HTTP 200