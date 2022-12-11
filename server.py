# -*- coding: utf-8 -*-

import http.server
from http.server import HTTPServer, BaseHTTPRequestHandler
import socketserver
import sys
import os

PORT = 8080

Handler = http.server.SimpleHTTPRequestHandler

Handler.extensions_map = {
    '.manifest': 'text/cache-manifest',
    '.html': 'text/html',
    '.png': 'image/png',
    '.jpg': 'image/jpg',
    '.svg':	'image/svg+xml',
    '.css':	'text/css',
    '.js':	'application/x-javascript',
    '.wasm':	'application/wasm',
    '': 'application/octet-stream',  # Default
}

os.chdir("static")
httpd = socketserver.TCPServer(("127.0.0.1", PORT), Handler)

print("Serving at http://localhost:" + str(PORT))
httpd.serve_forever()
