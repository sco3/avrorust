use jni::objects::{JClass, JString};
use jni::sys::jint;
use jni::sys::jlong;
use jni::JNIEnv;
mod av;

#[no_mangle]
pub extern "system" fn Java_com_tnf_kafka_serde_RustTest_zag(
	env: JNIEnv,
	class: JClass,
	aBuf: jlong,
	aLen: jlong,
	aOut: jlong,
) -> jint {
	let bbuf = unsafe { std::slice::from_raw_parts(aBuf as *const u8, aLen as usize) };
	let mut reader = av::AvroReader {
		buf: bbuf,
		len: aLen as usize,
		read: 0,
	};
	let mut out: i64 = 0;
	let result = av::read_long(&mut reader, &mut out);
	unsafe { *(aOut as *mut i64) = out };
	return result;
}
