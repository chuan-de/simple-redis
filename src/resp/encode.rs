use crate::{
    BulkString, RespArray, RespEncode, RespMap, RespNull, RespNullArray, RespNullBulkString,
    RespSet, SimpleError, SimpleString,
};
const BUF_CAP: usize = 4096;

impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}

impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(&format!("${}\r\n", self.len()).into_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

impl RespEncode for RespNullBulkString {
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }
}

impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("*{}\r\n", self.len()).into_bytes());
        for item in self.0 {
            buf.extend_from_slice(&item.encode());
        }
        buf
    }
}

impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}

impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        b"*-1\r\n".to_vec()
    }
}

impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self < 0 { "-" } else { "" };
        format!("${}", sign).into_bytes()
    }
}

impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        format!("#{}\r\n", if self { "t" } else { "f" }).into_bytes()
    }
}

impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(32);
        let ret = if self.abs() > 1e+8 {
            format!(",{:e}\r\n", self)
        } else {
            let sign = if self < 0.0 { "" } else { "+" };
            format!(",{}{}\r\n", sign, self)
        };
        buf.extend_from_slice(&ret.into_bytes());
        buf
    }
}

impl RespEncode for RespMap {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("%{}\r\n", self.len()).into_bytes());
        for (key, value) in self.0 {
            buf.extend_from_slice(SimpleString::new(key).encode().as_slice());
            buf.extend_from_slice(&value.encode());
        }
        buf
    }
}

impl RespEncode for RespSet {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("~{}\r\n", self.len()).into_bytes());
        for item in self.0 {
            buf.extend_from_slice(&item.encode());
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RespFrame;
    #[test]
    fn test_simple_string_encode() {
        let frame: RespFrame = SimpleString::new("OK".to_string()).into();
        assert_eq!(frame.encode(), b"+OK\r\n")
    }

    #[test]
    fn test_error_encode() {
        let frame: RespFrame = SimpleError::new("ERR".to_string()).into();
        assert_eq!(frame.encode(), b"-ERR\r\n")
    }

    #[test]
    fn test_integer_encode() {
        let frame: RespFrame = 123.into();
        assert_eq!(frame.encode(), b":123\r\n");
        let frame: RespFrame = (-123).into();
        assert_eq!(frame.encode(), b":-123\r\n");
    }

    #[test]
    fn test_bulk_string_encode() {
        let frame = BulkString::new(b"hello world".to_vec());
        assert_eq!(frame.encode(), b"$11\r\nhello world\r\n");
    }

    #[test]
    fn test_null_bulk_string_encode() {
        let frame = RespNullBulkString;
        assert_eq!(frame.encode(), b"$-1\r\n");
    }

    #[test]
    fn test_array_encode() {
        let frame = RespArray::new(vec![
            SimpleString::new("set".to_string()).into(),
            SimpleString::new("hello".to_string()).into(),
            SimpleString::new("world".to_string()).into(),
        ])
        .into();
        assert_eq!(
            frame.encode(),
            b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
        );
    }

    #[test]
    fn test_null_array_encode() {
        let frame = RespNullArray.into();
        assert_eq!(frame.encode(), b"*-1\r\n");
    }

    #[test]
    fn test_null_encode() {
        let frame = RespNull.into();
        assert_eq!(frame.encode(), b"_\r\n");
    }

    #[test]
    fn test_bool_encode() {
        let frame = true.into();
        assert_eq!(frame.encode(), b"+OK\r\n");
        let frame = false.into();
        assert_eq!(frame.encode(), b"-ERR\r\n");
    }

    #[test]
    fn test_double_encode() {
        let frame = 123.456.into();
        assert_eq!(frame.encode(), b":123.456\r\n");
        let frame = (-123.456).into();
        assert_eq!(frame.encode(), b":-123.456\r\n");
        let frame = 1.23456e+8.into();
        assert_eq!(frame.encode(), b":123456e+8\r\n");
        let frame = (-1.23456e-8).into();
        assert_eq!(frame.encode(), b":-123456e-8\r\n");
    }

    #[test]
}
