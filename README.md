# Ambient Weather Local

A local receiver for Ambient Weather data.

## How to use

1. Install [awnet](https://support.ambientweather.com/hc/en-us/articles/20093657840397-Ambient-Weather-APP-Download-Center)

   The awnet application is needed to configure your weather station console to
   send data to a local receiver.
1. Launch awnet and select "Next" until reaching the "Customized" screen
1. Enable customized output select "Ambient Weather" as the output style, set
   the hostname/IP address, the path to "/ambient-weather/upload?", the port to
   9111, and an update interval.
1. Save
1. Launch `ambient-weather-local` on the IP or hostname selected in step 3

`ambent-weather-local` binds to all interfaces on port 9111 by default.
Use `ambient-weather-local --address` to change this.

`ambient-weather-local` will save the last 20 reports sent from the weather
station.  This is five minutes with a 16 second update interval.  Use
`ambient-weather-local --max-reports` to change this.

## See also

[ambient-weather-exporter](https://github.com/trickv/ambient-weather-exporter)
which fetches weather data using the Ambient Weather API.
