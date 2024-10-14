use helio_pg::{models, PGClient, PGConn};

use crate::common;

struct InstanceType {
    pub cpu: i32,
    pub memory: i32,
    pub disk: i32,
}

const INSTANCE_TYPES: [InstanceType; 4] = [
    InstanceType {
        cpu: 1,
        memory: 1024,
        disk: 1024 * 8,
    },
    InstanceType {
        cpu: 1,
        memory: 2048,
        disk: 1024 * 10,
    },
    InstanceType {
        cpu: 2,
        memory: 4096,
        disk: 1024 * 20,
    },
    InstanceType {
        cpu: 4,
        memory: 8192,
        disk: 1024 * 50,
    },
];

pub fn start_instance(conn: &mut PGConn, uuid: String, created_by: String) -> Result<bool, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
    let instance = models::instance::Instance::_default_get_by_uuid(conn, uuid, created_by)?;

    let instance_type = &INSTANCE_TYPES[instance.itype as usize];

    let output = common::create_process(
        "qemu-system-x86_64",
        vec![
            "-enable-kvm", // KVM 사용

            "-machine", "pc-q35-7.2",
            "-cpu", "host",
            "-m", format!("{}", instance_type.memory).as_str(), // 메모리 크기 (예: 1024MB)
            "-smp", format!("cpus={}", instance_type.cpu).as_str(), // CPU 개수
            "-drive", format!("id=disk1,file=/etc/helio/disks/{}.qcow2,if=none,index=0", instance.uuid).as_str(), // 가상 디스크 경로
            "-device", "virtio-blk-pci,drive=disk1,bootindex=1",
            "-drive", "id=cd1,file=/etc/helio/images/archlinux-2024.10.01-x86_64.iso,if=none,index=1", // 가상 디스크 경로
            "-device", "virtio-blk-pci,drive=cd1,bootindex=2",
            // "-drive", format!("file={},media=cdrom,readonly=on", iso_path).as_str(), // 부팅 ISO 이미지 경로
            "-drive", "if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE.fd",
            "-netdev", "bridge,id=net0,br=br0",
            "-device", "virtio-net-pci,netdev=net0",
            "-vnc", format!("unix:/etc/helio/socket/{}.vnc", instance.uuid).as_str(), // VNC를 통해 가상 머신에 접속 가능 (포트 :1)
            "-pidfile", format!("/etc/helio/pids/{}.pid", instance.uuid).as_str(),
            "-chardev", format!("socket,id=qmp,path=/etc/helio/socket/{}.qmp,server=on,wait=off", instance.uuid).as_str(),
            "-mon", "chardev=qmp,mode=control",
            "-daemonize",
        ],
    );

    // 명령어 실행 결과를 UTF-8로 변환하여 출력
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // eprintln!("Error:\n{}", stderr);

		return Err(stderr.into());
    }

    Ok(output.status.success())
}
