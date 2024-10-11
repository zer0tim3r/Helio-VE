import socket
import json

def qmp_connect(socket_path):
    # Unix 소켓 생성
    client = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    
    try:
        # QMP 소켓에 연결
        client.connect(socket_path)
        print("Connected to QMP.")

        # 초기 QMP 환영 메시지 수신
        welcome_message = client.recv(1024).decode('utf-8')
        print("Received:", welcome_message)
        # QMP 환영 메시지를 JSON으로 파싱
        welcome = json.loads(welcome_message)
        
        if welcome["QMP"]:
            # QMP 초기화 명령 (qmp_capabilities) 전송
            qmp_cmd = json.dumps({"execute": "qmp_capabilities"})
            client.sendall(qmp_cmd.encode('utf-8'))
            
            # 응답 수신
            response = client.recv(2048).decode('utf-8')
            # print("Received:", response)

            status_cmd = json.dumps({ "execute": "query-jobs" })
            client.sendall(status_cmd.encode('utf-8'))
            
            # 응답 수신
            status_response = client.recv(16384).decode('utf-8')
            print("Received status:", status_response)

            status_cmd = json.dumps({"execute": "query-status"})
            client.sendall(status_cmd.encode('utf-8'))
            
            # 응답 수신
            status_response = client.recv(16384).decode('utf-8')
            print("Received status:", status_response)

            status_cmd = json.dumps({"execute": "cont"})
            client.sendall(status_cmd.encode('utf-8'))
            
            # 응답 수신
            status_response = client.recv(16384).decode('utf-8')
            print("Received status:", status_response)

    except Exception as e:
        print("Error:", e)
    finally:
        client.close()

if __name__ == "__main__":
    # Unix 소켓 경로 설정 (QEMU에서 사용 중인 경로로 변경 필요)
    socket_path = "/var/run/qemu/socket/example-vm.qmp"
    qmp_connect(socket_path)
