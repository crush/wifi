```
┌──────────────────────────────────────────────────────────────┐
│   / wifi                                                     │
│   wifi network manager for macos                             │
└──────────────────────────────────────────────────────────────┘
```

```bash
> install?

  curl -fsSL raw.githubusercontent.com/crush/wifi/main/i | sh

> usage?

  wifi              status (ip or ssid)
  wifi name         network name (slow on macos 15+)
  wifi list         select network
  wifi pass [name]  show password (needs keychain approval)
  wifi signal       signal strength
  wifi speed        speed test
  wifi on           turn wifi on
  wifi off          turn wifi off
  wifi <name>       connect to network

> stack?

  rust · crossterm · networksetup
```
