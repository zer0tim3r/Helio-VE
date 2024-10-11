use crate::common;

pub fn create_disk(id: &str, size: &str, format: &str) {
	let path = format!("/var/run/qemu/disks/{}.qcow2", id);

    let output = common::create_process("/usr/bin/qemu-img", vec![
		"create",
		"-f",
		format,
		path.as_str(),
		size,
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