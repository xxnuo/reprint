pub mod hasher {
    /// 获取字符串的Hash值，使用wyhash算法，种子为0
    ///
    pub fn hash(data: &str) -> String {
        use wyhash::wyhash;
        let hash_u64 = wyhash(data.as_bytes(), 0);
        return format!("{:x}",hash_u64)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_hasher_hash() {
        use super::hasher::hash;
        assert_eq!("7f2d8c6cde2b0d82",hash("1"));
        assert_eq!("c5938f0e9280a1bd",hash("“hello,world!你好，世界！&*（@\""));
        assert_eq!("475c437fd29f3a6d",hash("abcdefghijklmnopqrstuvwxyz1234567890"));
    }
}