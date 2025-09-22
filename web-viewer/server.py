#!/usr/bin/env python3

import http.server
import socketserver
import os
import sys
import threading
import webbrowser
from urllib.parse import urlparse

class WebViewerHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=os.path.dirname(os.path.abspath(__file__)), **kwargs)

    def log_message(self, format, *args):
        # Reduce log noise
        pass

    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', '*')
        super().end_headers()

def start_server(port):
    handler = WebViewerHandler
    with socketserver.TCPServer(("", port), handler) as httpd:
        print(f"Web viewer server running at http://localhost:{port}")
        httpd.serve_forever()

if __name__ == "__main__":
    PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 11721

    print(f"Starting web viewer server on port {PORT}")
    print(f"Open your browser to: http://localhost:{PORT}")

    # Start server in background
    server_thread = threading.Thread(target=start_server, args=(PORT,))
    server_thread.daemon = True
    server_thread.start()

    # Open browser after a short delay
    try:
        import time
        time.sleep(2)
        webbrowser.open(f'http://localhost:{PORT}')
        print(f"Opened browser to http://localhost:{PORT}")
    except Exception as e:
        print(f"Could not open browser automatically: {e}")
        print(f"Please manually open: http://localhost:{PORT}")

    # Keep the script running
    try:
        while True:
            import time
            time.sleep(1)
    except KeyboardInterrupt:
        print("\nShutting down server...")
        sys.exit(0)