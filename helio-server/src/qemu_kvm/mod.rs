use std::{fs::File, io::Read};

use helio_pg::models::instance::Instance;

use sysinfo::{Pid, System};

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

const INSTANCE_IMAGES: [&str; 1] = ["Arch-Linux-x86_64-cloudimg.qcow2"];

pub fn create_instance(
    instance: Instance,
) -> Result<(), Box<(dyn std::error::Error + Send + Sync + 'static)>> {
    let instance_type = &INSTANCE_TYPES[instance.itype as usize];

    std::fs::copy(
        format!(
            "/etc/helio/images/{}",
            INSTANCE_IMAGES[instance.image as usize]
        ),
        format!("/etc/helio/disks/{}.qcow2", instance.uuid),
    )?;

    let output = common::create_process(
        "qemu-img",
        vec![
            "resize",
            format!("/etc/helio/disks/{}.qcow2", instance.uuid).as_str(),
            format!("{}M", instance_type.disk).as_str(),
        ],
    );

    // 명령어 실행 결과를 UTF-8로 변환하여 출력
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // eprintln!("Error:\n{}", stderr);

        return Err(stderr.into());
    }

    Ok(())
}

pub fn delete_instance(
    instance: Instance,
) -> Result<(), Box<(dyn std::error::Error + Send + Sync + 'static)>> {
    match read_pid(format!("/etc/helio/pids/{}.pid", instance.uuid).as_str()) {
        Ok(pid) => {
            let s = System::new_all();
            match s.process(Pid::from(pid as usize)) {
                Some(p) => {
                    p.kill();
                }
                None => (),
            }
        }
        Err(_) => (),
    }

    let ipt = iptables::new(false).map_err(|e| e.to_string())?;

    let check_del =
        |table: &str, chain: &str, rule: &str| -> Result<(), Box<dyn std::error::Error>> {
            if ipt.exists(table, chain, rule)? {
                ipt.delete(table, chain, rule)?;
            }

            Ok(())
        };

    check_del(
        "filter",
        "FORWARD",
        format!(
            "-s {} -m mac --mac-source {} -j ACCEPT",
            instance.ipv4.clone(),
            instance.mac.clone()
        )
        .as_str(),
    )
    .map_err(|e| e.to_string())?;

    std::fs::remove_file(format!("/etc/helio/pids/{}.pid", instance.uuid)).unwrap();
    std::fs::remove_file(format!("/etc/helio/socket/{}.vnc", instance.uuid)).unwrap();
    std::fs::remove_file(format!("/etc/helio/disks/{}.qcow2", instance.uuid)).unwrap();

    Ok(())
}

pub fn start_instance(
    instance: Instance,
) -> Result<(), Box<(dyn std::error::Error + Send + Sync + 'static)>> {
    match read_pid(format!("/etc/helio/pids/{}.pid", instance.uuid).as_str()) {
        Ok(pid) => {
            let s = System::new_all();
            if s.process(Pid::from(pid as usize)).is_some() {
                return Err("Process Running".into());
            }
        }
        Err(_) => (),
    }

    let instance_type = &INSTANCE_TYPES[instance.itype as usize];

    let output = common::create_process(
        "qemu-system-x86_64",
        vec![
            "-enable-kvm", // KVM 사용
            "-machine",
            "pc-q35-7.2",
            "-cpu",
            "host",
            "-m",
            format!("{}", instance_type.memory).as_str(), // 메모리 크기 (예: 1024MB)
            "-smp",
            format!("cpus={}", instance_type.cpu).as_str(), // CPU 개수
            "-drive",
            format!(
                "id=disk1,file=/etc/helio/disks/{}.qcow2,if=none,index=0",
                instance.uuid
            )
            .as_str(), // 가상 디스크 경로
            "-device",
            "virtio-blk-pci,drive=disk1,bootindex=1",
            // "-drive", "id=cd1,file=/etc/helio/images/archlinux-2024.10.01-x86_64.iso,if=none,index=1", // 가상 디스크 경로
            // "-device", "virtio-blk-pci,drive=cd1,bootindex=2",
            // "-drive", format!("file={},media=cdrom,readonly=on", iso_path).as_str(), // 부팅 ISO 이미지 경로
            "-drive",
            "if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE.fd",
            "-netdev",
            "bridge,id=net0,br=br0",
            "-device",
            format!("virtio-net-pci,netdev=net0,mac={}", instance.mac).as_str(),
            "-vnc",
            format!("unix:/etc/helio/socket/{}.vnc", instance.uuid).as_str(), // VNC를 통해 가상 머신에 접속 가능 (포트 :1)
            "-pidfile",
            format!("/etc/helio/pids/{}.pid", instance.uuid).as_str(),
            "-chardev",
            format!(
                "socket,id=qmp,path=/etc/helio/socket/{}.qmp,server=on,wait=off",
                instance.uuid
            )
            .as_str(),
            "-mon",
            "chardev=qmp,mode=control",
            "-daemonize",
        ],
    );

    // 명령어 실행 결과를 UTF-8로 변환하여 출력
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // eprintln!("Error:\n{}", stderr);

        return Err(stderr.into());
    }

    Ok(())
}

fn read_pid(file_path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // 파일 열기
    let mut file = File::open(file_path)?;

    // 파일 내용을 저장할 문자열 버퍼
    let mut contents = String::new();

    // 파일 내용 읽기
    file.read_to_string(&mut contents)?;

    // 문자열을 정수로 변환
    let number = i32::from_str_radix(&contents.trim(), 10)?;

    Ok(number)
}
