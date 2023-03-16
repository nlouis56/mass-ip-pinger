import socket
import time

class ConnectionTester():
    def __init__(self):
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.socket.settimeout(1)

    def test_connection(self, host, port):
        try:
            self.socket.connect((host, port))
            self.socket.close()
            return True
        except socket.error:
            return False

    def is_valid_ip(self, ip):
        try:
            socket.inet_aton(ip)
            return True
        except:
            return False

    def is_up(self, host):
        start = time.perf_counter_ns()
        try:
            if not self.is_valid_ip(host):
                return False
            else:
                conn = socket.gethostbyaddr(host)
                end = time.perf_counter_ns()
                elapsed = (end - start) / 1000000
                return conn, elapsed
        except socket.herror:
            return False

if __name__ == "__main__":
    ct = ConnectionTester()
    print(ct.is_up('216.58.213.78'))