extern crate jni;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use jni::JNIEnv;
use jni::objects::{GlobalRef, JClass, JObject, JString};
use jni::sys::{jint, jlong};

// jstring : 수명이 없는 원시포인터
// JString : 수명이 있는 객체
#[no_mangle]
pub extern "system" fn Java_HelloWorld_hello<'local>(
    // JNIEnv : 자바와 상호작용하는데 필요한 모든 기능을 제공하는 인터페이스
    mut env: JNIEnv<'local>,
    // 정적 메서드를 가지고 있는 클래스 사용하진 않음
    _class: JClass<'local>,
    // 인자
    name: JString<'local>,
) -> JString<'local> {
    let input: String = env.get_string(&name).unwrap().into();
    let output = env.new_string(format!("Hello, {input}!")).unwrap();

    output
}

#[no_mangle]
pub extern "system" fn Java_HelloWorld_callback<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    n: jint,
    callback: JObject,
    name: JString<'local>,
) {
    let i = n as i32;
    let res: jint = (2..i + 1).product();

    let input: String = env.get_string(&name).unwrap().into();
    env.call_method(callback, input, "(I)V", &[res.into()]).unwrap();
}

struct Counter {
    count: i32,
    callback: GlobalRef,
    method_name: String
}

impl Counter {
    pub fn new(callback: GlobalRef, method_name: String) -> Counter {
        Counter {
            count: 0,
            callback,
            method_name
        }
    }

    pub fn increment(&mut self, env: &mut JNIEnv) {
        self.count = self.count + 1;
        env.call_method(
            &self.callback,
            &self.method_name,
            "(I)V",
            &[self.count.into()],
        ).unwrap();
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_HelloWorld_counterNew(
    mut env: JNIEnv,
    _class: JClass,
    callback: JObject,
    method_name: JString,
) -> jlong {
    let global_ref = env.new_global_ref(callback).unwrap();
    let counter = Counter::new(
        global_ref,
        env.get_string(&method_name).unwrap().into(),
    );

    Box::into_raw(Box::new(counter)) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_HelloWorld_counterIncrement(
    mut env: JNIEnv,
    _class: JClass,
    counter_ptr: jlong,
) {
    let counter = &mut *(counter_ptr as *mut Counter);

    counter.increment(&mut env);
}

#[no_mangle]
pub unsafe extern "system" fn Java_HelloWorld_counterDestroy(
    mut env: JNIEnv,
    _class: JClass,
    counter_ptr: jlong,
) {
    let _boxed_counter = Box::from_raw(counter_ptr as *mut Counter);
}

#[no_mangle]
pub extern "system" fn Java_HelloWorld_asyncComputation(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) {
    // JNIEnv는 스레드환겨에서 사용할 수 없어 JavaVM을 가져와야 한다.
    // JavaVM은 JNIEnv와 달리 Send Trait을 구현해 있다.
    let jvm = env.get_java_vm().unwrap();

    // GC에 의해 사라지지 않기위해 전역참조를 진행 함
    let callback = env.new_global_ref(callback).unwrap();

    // 채널을 사용하여 스레드가 시작되기전에 프로그램이 종료되는걸 방지 함
    let (tx, rx) = mpsc::channel();

    let _ = thread::spawn(move || {
        // Thread시작 시그널
        tx.send(()).unwrap();

        // JavaVM을 이용해 JNIEnv에 접근
        let mut env = jvm.attach_current_thread().unwrap();

        for i in 0..11{
            let progress = (i * 10) as jint;

            env.call_method(&callback, "asyncCallback", "(I)V", &[progress.into()]).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    rx.recv().unwrap();
}