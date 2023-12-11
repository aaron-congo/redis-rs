#[test]
fn test_is_single_arg() {
    use redis::ToRedisArgs;

    let sslice: &[_] = &["foo"][..];
    let nestslice: &[_] = &[sslice][..];
    let nestvec = vec![nestslice];
    let bytes = b"Hello World!";
    let twobytesslice: &[_] = &[bytes, bytes][..];
    let twobytesvec = vec![bytes, bytes];

    assert!("foo".is_single_arg());
    assert!(sslice.is_single_arg());
    assert!(nestslice.is_single_arg());
    assert!(nestvec.is_single_arg());
    assert!(bytes.is_single_arg());

    assert!(!twobytesslice.is_single_arg());
    assert!(!twobytesvec.is_single_arg());
}

#[test]
fn test_info_dict() {
    use redis::{FromRedisValue, InfoDict, Value};

    let d: InfoDict = FromRedisValue::from_redis_value(&Value::SimpleString(
        "# this is a comment\nkey1:foo\nkey2:42\n".into(),
    ))
    .unwrap();

    assert_eq!(d.get("key1"), Some("foo".to_string()));
    assert_eq!(d.get("key2"), Some(42i64));
    assert_eq!(d.get::<String>("key3"), None);
}

#[test]
fn test_i32() {
    use redis::{ErrorKind, FromRedisValue, Value};

    let i = FromRedisValue::from_redis_value(&Value::SimpleString("42".into()));
    assert_eq!(i, Ok(42i32));

    let i = FromRedisValue::from_redis_value(&Value::Int(42));
    assert_eq!(i, Ok(42i32));

    let i = FromRedisValue::from_redis_value(&Value::BulkString("42".into()));
    assert_eq!(i, Ok(42i32));

    let bad_i: Result<i32, _> =
        FromRedisValue::from_redis_value(&Value::SimpleString("42x".into()));
    assert_eq!(bad_i.unwrap_err().kind(), ErrorKind::TypeError);
}

#[test]
fn test_u32() {
    use redis::{ErrorKind, FromRedisValue, Value};

    let i = FromRedisValue::from_redis_value(&Value::SimpleString("42".into()));
    assert_eq!(i, Ok(42u32));

    let bad_i: Result<u32, _> = FromRedisValue::from_redis_value(&Value::SimpleString("-1".into()));
    assert_eq!(bad_i.unwrap_err().kind(), ErrorKind::TypeError);
}

#[test]
fn test_vec() {
    use redis::{FromRedisValue, Value};

    let v = FromRedisValue::from_redis_value(&Value::Array(vec![
        Value::BulkString("1".into()),
        Value::BulkString("2".into()),
        Value::BulkString("3".into()),
    ]));
    assert_eq!(v, Ok(vec![1i32, 2, 3]));

    let content: &[u8] = b"\x01\x02\x03\x04";
    let content_vec: Vec<u8> = Vec::from(content);
    let v = FromRedisValue::from_redis_value(&Value::BulkString(content_vec.clone()));
    assert_eq!(v, Ok(content_vec));

    let content: &[u8] = b"1";
    let content_vec: Vec<u8> = Vec::from(content);
    let v = FromRedisValue::from_redis_value(&Value::BulkString(content_vec.clone()));
    assert_eq!(v, Ok(vec![b'1']));
    let v = FromRedisValue::from_redis_value(&Value::BulkString(content_vec));
    assert_eq!(v, Ok(vec![1_u16]));
}

#[test]
fn test_box_slice() {
    use redis::{FromRedisValue, Value};

    let v = FromRedisValue::from_redis_value(&Value::Array(vec![
        Value::BulkString("1".into()),
        Value::BulkString("2".into()),
        Value::BulkString("3".into()),
    ]));
    assert_eq!(v, Ok(vec![1i32, 2, 3].into_boxed_slice()));

    let content: &[u8] = b"\x01\x02\x03\x04";
    let content_vec: Vec<u8> = Vec::from(content);
    let v = FromRedisValue::from_redis_value(&Value::BulkString(content_vec.clone()));
    assert_eq!(v, Ok(content_vec.into_boxed_slice()));

    let content: &[u8] = b"1";
    let content_vec: Vec<u8> = Vec::from(content);
    let v = FromRedisValue::from_redis_value(&Value::BulkString(content_vec.clone()));
    assert_eq!(v, Ok(vec![b'1'].into_boxed_slice()));
    let v = FromRedisValue::from_redis_value(&Value::BulkString(content_vec));
    assert_eq!(v, Ok(vec![1_u16].into_boxed_slice()));

    assert_eq!(
        Box::<[i32]>::from_redis_value(
            &Value::BulkString("just a string".into())
        ).unwrap_err().to_string(),
        "Response was of incompatible type - TypeError: \"Conversion to alloc::boxed::Box<[i32]> failed.\" (response was bulk-string('\"just a string\"'))",
    );
}

#[test]
fn test_arc_slice() {
    use redis::{FromRedisValue, Value};
    use std::sync::Arc;

    let v = FromRedisValue::from_redis_value(&Value::Array(vec![
        Value::BulkString("1".into()),
        Value::BulkString("2".into()),
        Value::BulkString("3".into()),
    ]));
    assert_eq!(v, Ok(Arc::from(vec![1i32, 2, 3])));

    let content: &[u8] = b"\x01\x02\x03\x04";
    let content_vec: Vec<u8> = Vec::from(content);
    let v = FromRedisValue::from_redis_value(&Value::BulkString(content_vec.clone()));
    assert_eq!(v, Ok(Arc::from(content_vec)));

    let content: &[u8] = b"1";
    let content_vec: Vec<u8> = Vec::from(content);
    let v = FromRedisValue::from_redis_value(&Value::BulkString(content_vec.clone()));
    assert_eq!(v, Ok(Arc::from(vec![b'1'])));
    let v = FromRedisValue::from_redis_value(&Value::BulkString(content_vec));
    assert_eq!(v, Ok(Arc::from(vec![1_u16])));

    assert_eq!(
        Arc::<[i32]>::from_redis_value(
            &Value::BulkString("just a string".into())
        ).unwrap_err().to_string(),
        "Response was of incompatible type - TypeError: \"Conversion to alloc::sync::Arc<[i32]> failed.\" (response was bulk-string('\"just a string\"'))",
    );
}

#[test]
fn test_single_bool_vec() {
    use redis::{FromRedisValue, Value};

    let v = FromRedisValue::from_redis_value(&Value::BulkString("1".into()));

    assert_eq!(v, Ok(vec![true]));
}

#[test]
fn test_single_i32_vec() {
    use redis::{FromRedisValue, Value};

    let v = FromRedisValue::from_redis_value(&Value::BulkString("1".into()));

    assert_eq!(v, Ok(vec![1i32]));
}

#[test]
fn test_single_u32_vec() {
    use redis::{FromRedisValue, Value};

    let v = FromRedisValue::from_redis_value(&Value::BulkString("42".into()));

    assert_eq!(v, Ok(vec![42u32]));
}

#[test]
fn test_single_string_vec() {
    use redis::{FromRedisValue, Value};

    let v = FromRedisValue::from_redis_value(&Value::BulkString("1".into()));

    assert_eq!(v, Ok(vec!["1".to_string()]));
}

#[test]
fn test_tuple() {
    use redis::{FromRedisValue, Value};

    let v = FromRedisValue::from_redis_value(&Value::Array(vec![Value::Array(vec![
        Value::BulkString("1".into()),
        Value::BulkString("2".into()),
        Value::BulkString("3".into()),
    ])]));

    assert_eq!(v, Ok(((1i32, 2, 3,),)));
}

#[test]
fn test_hashmap() {
    use fnv::FnvHasher;
    use redis::{FromRedisValue, Value};
    use std::collections::HashMap;
    use std::hash::BuildHasherDefault;

    type Hm = HashMap<String, i32>;

    let v: Result<Hm, _> = FromRedisValue::from_redis_value(&Value::Array(vec![
        Value::BulkString("a".into()),
        Value::BulkString("1".into()),
        Value::BulkString("b".into()),
        Value::BulkString("2".into()),
        Value::BulkString("c".into()),
        Value::BulkString("3".into()),
    ]));
    let mut e: Hm = HashMap::new();
    e.insert("a".into(), 1);
    e.insert("b".into(), 2);
    e.insert("c".into(), 3);
    assert_eq!(v, Ok(e));

    type Hasher = BuildHasherDefault<FnvHasher>;
    type HmHasher = HashMap<String, i32, Hasher>;
    let v: Result<HmHasher, _> = FromRedisValue::from_redis_value(&Value::Array(vec![
        Value::BulkString("a".into()),
        Value::BulkString("1".into()),
        Value::BulkString("b".into()),
        Value::BulkString("2".into()),
        Value::BulkString("c".into()),
        Value::BulkString("3".into()),
    ]));

    let fnv = Hasher::default();
    let mut e: HmHasher = HashMap::with_hasher(fnv);
    e.insert("a".into(), 1);
    e.insert("b".into(), 2);
    e.insert("c".into(), 3);
    assert_eq!(v, Ok(e));
}

#[test]
fn test_bool() {
    use redis::{ErrorKind, FromRedisValue, Value};

    let v = FromRedisValue::from_redis_value(&Value::BulkString("1".into()));
    assert_eq!(v, Ok(true));

    let v = FromRedisValue::from_redis_value(&Value::BulkString("0".into()));
    assert_eq!(v, Ok(false));

    let v: Result<bool, _> = FromRedisValue::from_redis_value(&Value::BulkString("garbage".into()));
    assert_eq!(v.unwrap_err().kind(), ErrorKind::TypeError);

    let v = FromRedisValue::from_redis_value(&Value::SimpleString("1".into()));
    assert_eq!(v, Ok(true));

    let v = FromRedisValue::from_redis_value(&Value::SimpleString("0".into()));
    assert_eq!(v, Ok(false));

    let v: Result<bool, _> =
        FromRedisValue::from_redis_value(&Value::SimpleString("garbage".into()));
    assert_eq!(v.unwrap_err().kind(), ErrorKind::TypeError);

    let v = FromRedisValue::from_redis_value(&Value::Okay);
    assert_eq!(v, Ok(true));

    let v = FromRedisValue::from_redis_value(&Value::Nil);
    assert_eq!(v, Ok(false));

    let v = FromRedisValue::from_redis_value(&Value::Int(0));
    assert_eq!(v, Ok(false));

    let v = FromRedisValue::from_redis_value(&Value::Int(42));
    assert_eq!(v, Ok(true));
}

#[cfg(feature = "bytes")]
#[test]
fn test_bytes() {
    use bytes::Bytes;
    use redis::{ErrorKind, FromRedisValue, RedisResult, Value};

    let content: &[u8] = b"\x01\x02\x03\x04";
    let content_vec: Vec<u8> = Vec::from(content);
    let content_bytes = Bytes::from_static(content);

    let v: RedisResult<Bytes> = FromRedisValue::from_redis_value(&Value::BulkString(content_vec));
    assert_eq!(v, Ok(content_bytes));

    let v: RedisResult<Bytes> =
        FromRedisValue::from_redis_value(&Value::SimpleString("garbage".into()));
    assert_eq!(v.unwrap_err().kind(), ErrorKind::TypeError);

    let v: RedisResult<Bytes> = FromRedisValue::from_redis_value(&Value::Okay);
    assert_eq!(v.unwrap_err().kind(), ErrorKind::TypeError);

    let v: RedisResult<Bytes> = FromRedisValue::from_redis_value(&Value::Nil);
    assert_eq!(v.unwrap_err().kind(), ErrorKind::TypeError);

    let v: RedisResult<Bytes> = FromRedisValue::from_redis_value(&Value::Int(0));
    assert_eq!(v.unwrap_err().kind(), ErrorKind::TypeError);

    let v: RedisResult<Bytes> = FromRedisValue::from_redis_value(&Value::Int(42));
    assert_eq!(v.unwrap_err().kind(), ErrorKind::TypeError);
}

#[test]
fn test_cstring() {
    use redis::{ErrorKind, FromRedisValue, RedisResult, Value};
    use std::ffi::CString;

    let content: &[u8] = b"\x01\x02\x03\x04";
    let content_vec: Vec<u8> = Vec::from(content);

    let v: RedisResult<CString> = FromRedisValue::from_redis_value(&Value::BulkString(content_vec));
    assert_eq!(v, Ok(CString::new(content).unwrap()));

    let v: RedisResult<CString> =
        FromRedisValue::from_redis_value(&Value::SimpleString("garbage".into()));
    assert_eq!(v, Ok(CString::new("garbage").unwrap()));

    let v: RedisResult<CString> = FromRedisValue::from_redis_value(&Value::Okay);
    assert_eq!(v, Ok(CString::new("OK").unwrap()));

    let v: RedisResult<CString> =
        FromRedisValue::from_redis_value(&Value::SimpleString("gar\0bage".into()));
    assert_eq!(v.unwrap_err().kind(), ErrorKind::TypeError);

    let v: RedisResult<CString> = FromRedisValue::from_redis_value(&Value::Nil);
    assert_eq!(v.unwrap_err().kind(), ErrorKind::TypeError);

    let v: RedisResult<CString> = FromRedisValue::from_redis_value(&Value::Int(0));
    assert_eq!(v.unwrap_err().kind(), ErrorKind::TypeError);

    let v: RedisResult<CString> = FromRedisValue::from_redis_value(&Value::Int(42));
    assert_eq!(v.unwrap_err().kind(), ErrorKind::TypeError);
}

#[test]
fn test_types_to_redis_args() {
    use redis::ToRedisArgs;
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;
    use std::collections::HashMap;
    use std::collections::HashSet;

    assert!(!5i32.to_redis_args().is_empty());
    assert!(!"abc".to_redis_args().is_empty());
    assert!(!"abc".to_redis_args().is_empty());
    assert!(!String::from("x").to_redis_args().is_empty());

    assert!(![5, 4]
        .iter()
        .cloned()
        .collect::<HashSet<_>>()
        .to_redis_args()
        .is_empty());

    assert!(![5, 4]
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>()
        .to_redis_args()
        .is_empty());

    // this can be used on something HMSET
    assert!(![("a", 5), ("b", 6), ("C", 7)]
        .iter()
        .cloned()
        .collect::<BTreeMap<_, _>>()
        .to_redis_args()
        .is_empty());

    // this can also be used on something HMSET
    assert!(![("d", 8), ("e", 9), ("f", 10)]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>()
        .to_redis_args()
        .is_empty());
}

#[test]
fn test_attributes() {
    use redis::{parse_redis_value, FromRedisValue, Value};
    let bytes: &[u8] = b"*3\r\n:1\r\n:2\r\n|1\r\n+ttl\r\n:3600\r\n:3\r\n";
    let val = parse_redis_value(bytes).unwrap();
    {
        // The case user doesn't expect attributes from server
        let x: Vec<i32> = redis::FromRedisValue::from_redis_value(&val).unwrap();
        assert_eq!(x, vec![1, 2, 3]);
    }
    {
        // The case user wants raw value from server
        let x: Value = FromRedisValue::from_redis_value(&val).unwrap();
        assert_eq!(
            x,
            Value::Array(vec![
                Value::Int(1),
                Value::Int(2),
                Value::Attribute {
                    data: Box::new(Value::Int(3)),
                    attributes: vec![(Value::SimpleString("ttl".to_string()), Value::Int(3600))]
                }
            ])
        )
    }
}
