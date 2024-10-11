import psutil

def get_qemu_cpu_usage(qemu_pid):
    # QEMU 프로세스 정보 가져오기
    process = psutil.Process(qemu_pid)
    
    try:
        # CPU 사용량 (퍼센트)
        cpu_usage = process.cpu_percent(interval=1.0)
        print(f"QEMU Process {qemu_pid} CPU Usage: {cpu_usage}%")
    except psutil.NoSuchProcess:
        print(f"No process with PID {qemu_pid} found.")

if __name__ == "__main__":
    with open("/var/run/qemu/pids/example-vm.pid", "r") as fd:
        pid = int(fd.read().strip())
        while True:
            get_qemu_cpu_usage(pid)
        fd.close()