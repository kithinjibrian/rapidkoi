# TLS fixtures (tests only)

Made by `make.sh` (OpenSSL 3.4 or newer) for PRP-009's tests:

| File | What |
| --- | --- |
| `ca.pem` | A throwaway CA, "Rapidkoi test CA (tests only)". Its key was deleted after signing. |
| `localhost.pem`, `localhost.key` | A certificate for `localhost` and `127.0.0.1`, signed by that CA, valid for 100 years |
| `wrong-host.pem`, `wrong-host.key` | A certificate for `wrong.example`: refused for `localhost` |
| `expired.pem`, `expired.key` | For `localhost`, but valid only on 1 January 2020: refused as expired |

Only the tests trust `ca.pem`, through `dialing.Network { ca_file }`. Rapidkoi itself never takes a CA file from anywhere: no config, environment variable or flag. These keys are not secrets: they protect nothing but test servers on 127.0.0.1.
