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

  wifi              show current network
  wifi list         list networks (j/k to navigate, enter to connect)
  wifi pass         show password for current network
  wifi pass <name>  show password for specific network
  wifi signal       live signal strength monitor
  wifi speed        test download speed
  wifi <name>       connect to network
  wifi on           turn wifi on
  wifi off          turn wifi off

> features?

  ✓ show current network
  ✓ list available networks with interactive selection
  ✓ connect to networks with password support
  ✓ retrieve saved wifi passwords from keychain
  ✓ live signal strength monitor with bars
  ✓ download speed test
  ✓ turn wifi on/off

> stack?

  rust · crossterm · networksetup · security
```
