use crate::common;

pub fn create_disk(id: &str, size: &str, format: &str) -> bool {
	let path = format!("/var/run/qemu/disks/{}.qcow2", id);

    let output = common::create_process("/usr/bin/qemu-img", vec![
		"create",
		"-f",
		format,
		path.as_str(),
		size,
    ]);

    output.status.success()
}