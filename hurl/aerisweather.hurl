# Use it for current and forecast conditions in hourly manner
GET https://api.aerisapi.com/conditions/zaporizhzhia,ukraine
[QueryStringParams]
client_id: {{AERISWEATHER_CLIENT_ID}}
client_secret: {{AERISWEATHER_CLIENT_SECRET}}
format: json
plimit: 1
filter: 1min
from: today
to: tomorrow


HTTP 200
# Use it for current and forecast conditions in daily manner

GET https://api.aerisapi.com/conditions/summary/zaporizhzhia,ukraine
[QueryStringParams]
client_id: {{AERISWEATHER_CLIENT_ID}}
client_secret: {{AERISWEATHER_CLIENT_SECRET}}
format: json
from: today
to: tomorrow

HTTP 200
