import time

start = time.perf_counter_ns()
time.sleep(1)
end = time.perf_counter_ns()
elapsed = end - start
print(elapsed)