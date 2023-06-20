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