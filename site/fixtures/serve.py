#!/usr/bin/env python3
"""Serve the page with a stand-in for the service's sign-in routes, to see
Me in each state by hand or in a headless browser. Not the service: no
GitHub, no cookies, and one sign-in state for every browser.

    python3 site/fixtures/serve.py --state signed-in   # or signed-out, opted-out, none

`none` answers 404 on every service route, as GitHub Pages does. The page's
own calls move the state as the service would: signing in (GET /auth/login)
signs in, sign-out and delete sign out, opt-out opts out. Like the service,
a call that changes something needs an Origin equal to this server's own.
GET /__mock/state?set=<state> resets the state between checks. Open it as
127.0.0.1 (not localhost): any other Host is refused.
"""

import argparse
import json
import os
from datetime import datetime, timezone
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import parse_qs, urlsplit

HERE = os.path.dirname(os.path.abspath(__file__))
STATES = ("none", "signed-out", "signed-in", "opted-out")


class Handler(SimpleHTTPRequestHandler):
    state = "signed-out"
    data = b""
    me = {}
    speed = b""
    speed_status = 200
    host = ""  # "127.0.0.1:<port>": the only Host accepted, and the Origin a change needs

    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=os.path.dirname(HERE), **kwargs)

    def send(self, status, body=b"", kind="application/json", headers=()):
        self.send_response(status)
        if body:
            self.send_header("Content-Type", kind)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "private, no-store")
        for name, value in headers:
            self.send_header(name, value)
        self.end_headers()
        if body:
            self.wfile.write(body)

    def signed_in(self):
        return Handler.state in ("signed-in", "opted-out")

    def foreign_host(self):
        """Refuse a request for any other host name, as a DNS-rebinding page would send."""
        if self.headers.get("Host") == Handler.host:
            return False
        self.send(421, b'{"error":"host"}')
        return True

    def do_GET(self):
        if self.foreign_host():
            return None
        url = urlsplit(self.path)
        path = url.path
        if path == "/dashboard.json":
            return self.send(200, Handler.data)
        if path == "/__mock/state":
            state = parse_qs(url.query).get("set", [""])[0]
            if state not in STATES:
                return self.send(400, b'{"error":"state"}')
            Handler.state = state
            return self.send(204)
        if path.startswith("/auth/") or path.startswith("/api/v1/me"):
            if Handler.state == "none":
                return self.send(404, b'{"error":"not found"}')
            if path == "/auth/login":
                Handler.state = "signed-in" if Handler.state != "opted-out" else "opted-out"
                return self.send(302, headers=[("Location", "/#/me")])
            if path == "/api/v1/me":
                if not self.signed_in():
                    return self.send(401, b'{"error":"not signed in"}')
                me = dict(Handler.me, opted_out=Handler.state == "opted-out")
                return self.send(200, json.dumps(me).encode())
            if path == "/api/v1/me/speed":
                if not self.signed_in():
                    return self.send(401, b'{"error":"not signed in"}')
                if Handler.state == "opted-out":
                    return self.send(200, b'{"opted_out":true}')
                if Handler.speed_status != 200:
                    return self.send(Handler.speed_status, b'{"error":"mock"}')
                return self.send(200, Handler.speed)
            return self.send(404, b'{"error":"not found"}')
        return super().do_GET()

    def change(self, route):
        """POST /auth/logout, POST /api/v1/me/opt-out, DELETE /api/v1/me."""
        if Handler.state == "none":
            return self.send(404, b'{"error":"not found"}')
        origin = self.headers.get("Origin")
        if origin != f"http://{Handler.host}":
            self.log_message("refused: Origin %r", origin)
            return self.send(403, b'{"error":"origin"}')
        if route == "logout":
            Handler.state = "signed-out"
            return self.send(204)
        if not self.signed_in():
            return self.send(401, b'{"error":"not signed in"}')
        Handler.state = "opted-out" if route == "opt-out" else "signed-out"
        return self.send(204)

    def do_POST(self):
        if self.foreign_host():
            return None
        routes = {"/auth/logout": "logout", "/api/v1/me/opt-out": "opt-out"}
        path = urlsplit(self.path).path
        return self.change(routes[path]) if path in routes else self.send(405)

    def do_DELETE(self):
        if self.foreign_host():
            return None
        path = urlsplit(self.path).path
        return self.change("delete") if path == "/api/v1/me" else self.send(405)


def main():
    p = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    p.add_argument("--port", type=int, default=8770)
    p.add_argument("--state", choices=STATES, default="signed-out")
    p.add_argument("--data", default=os.path.join(HERE, "dashboard.sample.json"))
    p.add_argument("--me", default=os.path.join(HERE, "me.sample.json"))
    p.add_argument("--speed", default=os.path.join(HERE, "speed.sample.json"))
    p.add_argument("--speed-status", type=int, default=200, help="answer the speed route with this status instead")
    a = p.parse_args()
    with open(a.data) as f:
        data = json.load(f)
    # Fresh data, so the page shows no staleness banner over the states.
    data["generated_at"] = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.000000Z")
    Handler.data = json.dumps(data).encode()
    with open(a.me) as f:
        Handler.me = json.load(f)
    with open(a.speed, "rb") as f:
        Handler.speed = f.read()
    Handler.speed_status = a.speed_status
    Handler.state = a.state
    Handler.host = f"127.0.0.1:{a.port}"
    print(f"http://127.0.0.1:{a.port}/#/me  ({a.state})", flush=True)
    ThreadingHTTPServer(("127.0.0.1", a.port), Handler).serve_forever()


if __name__ == "__main__":
    main()
