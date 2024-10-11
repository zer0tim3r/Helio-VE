use crate::common;



pub fn create_instance(id: &str, cpus: i32, memory: i32, iso_path: &str) {
	let image_path = format!("/var/run/qemu/disks/{}.qcow2", id);

    let output = common::create_process("/usr/libexec/qemu-kvm", vec![
		"-enable-kvm",           // KVM 사용
		"-machine", "pc-q35-rhel9.4.0",
		"-cpu", "host",
		"-m", format!("{}", memory).as_str(), // 메모리 크기 (예: 1024MB)
		"-smp", format!("cpus={}", cpus).as_str(), // CPU 개수
		"-drive", format!("file={},format=qcow2,if=virtio,index=0", image_path).as_str(), // 가상 디스크 경로
		"-drive", format!("file={},media=cdrom,readonly=on", iso_path).as_str(), // 부팅 ISO 이미지 경로
		"-boot", "order=cd", // HDD 우선, 다음 CD
		"-drive", "if=pflash,format=raw,readonly=on,file=/usr/share/edk2/ovmf/OVMF_CODE.fd",
		"-netdev", "bridge,id=net0,br=br0",
		"-device", "virtio-net-pci,netdev=net0",
		"-vnc", format!("unix:/var/run/qemu/socket/{}.vnc", id).as_str(), // VNC를 통해 가상 머신에 접속 가능 (포트 :1)
		"-pidfile", format!("/var/run/qemu/pids/{}.pid", id).as_str(),
		"-chardev", format!("socket,id=qmp,path=/var/run/qemu/socket/{}.qmp,server=on,wait=off", id).as_str(),
		"-mon", "chardev=qmp,mode=control",
		"-daemonize"
    ]);

    // 명령어 실행 결과를 UTF-8로 변환하여 출력
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Output:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error:\n{}", stderr);
    }
}