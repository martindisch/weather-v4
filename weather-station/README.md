# weather-station

This is intended to run on a Raspberry Pi with the AM2302 sensor connected on
GPIO4. The pin can be changed in `src/main.rs`.

Because the `dht22_pi` library sets the scheduler priority for more accurate
timings, it requires the `cap_sys_nice` capability. This means you need to run

```console
# setcap 'cap_sys_nice=eip' target/release/weather-station
```

as root/with `sudo` on your binary after the build.
