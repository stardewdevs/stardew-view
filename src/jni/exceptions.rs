use jni::JNIEnv;
use jni::errors::Error;

pub fn throw_io_exception(env: &JNIEnv, msg: &str) -> Result<(), Error> {
    env.throw_new("java/io/IOException", msg)?;
    Ok(())
}
