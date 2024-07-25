const BASE64_TABLE : [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
];


pub fn encode(data: &str) -> String {
    let mut binary_str = String::with_capacity(data.len() * 8);
    for byte in data.bytes(){
        binary_str.push_str(&format!("{:08b}", byte));
    }
    
    if binary_str.len() % 6 != 0 {
        pad_to_by(&mut binary_str, 6, '0');
    }
    
    let mut encoded_str = String::new();
    for i in 0..binary_str.len() / 6 {
        let chunk = &binary_str[i*6..(i + 1) * 6];
        let index = usize::from_str_radix(chunk, 2).unwrap();
        encoded_str.push(BASE64_TABLE[index]);
    }
    
    if encoded_str.len() % 4 != 0{
        pad_to_by(&mut encoded_str, 4, '=');
    }
    
    encoded_str
}

fn pad_to_by(uneven_str: &mut String, to: usize, by: char){
    while uneven_str.len() % to != 0 {
        uneven_str.push(by);
    }
}

pub fn decode(data: &str) -> String {
    let stripped_data = data.trim_end_matches('=');
    
    let mut binary_str = String::new();
    for char in stripped_data.chars(){
        let table_index = BASE64_TABLE.iter().position(|&table_char| {
            table_char == char
        }).unwrap();
        binary_str.push_str(&format!("{table_index:06b}"));
    }
    
    let mut bytes = Vec::new();
    for i in 0..(binary_str.len() / 8) {
        let chunk_8bit = &binary_str[i * 8..(i + 1) * 8];
        let byte = u8::from_str_radix(chunk_8bit, 2).unwrap();
        bytes.push(byte);
    }

    String::from_utf8(bytes).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            String::from("aGVsbG90aGVyZXVpamtsbw=="),
            encode("hellothereuijklo")
        );

        assert_eq!(String::from("c3VybmFqYW4="), encode("surnajan"));
        assert_eq!(String::from("Ym90dGwxMjEy"), encode("bottl1212"));
        assert_eq!(
            String::from("QXNwaGFsdEA4OTA="),
            encode("Asphalt@890")
        );
        assert_eq!(
            String::from("KHNhUy5zYXNkYXcvd2Fpb25za2F3K2FzYXM9c2FkYXdheiNhc2R3"),
            encode("(saS.sasdaw/waionskaw+asas=sadawaz#asdw")
        );
        assert_eq!(
            String::from("ODkyM2prbXdlcDItPXBva21hczEuLC9hczthbHNtYWQ="),
            encode("8923jkmwep2-=pokmas1.,/as;alsmad")
        );
        assert_eq!(
            String::from("XHBvcG9rZXdubXBjITJrc21kU2xta3Ns"),
            encode("\\popokewnmpc!2ksmdSlmksl")
        );

        assert_eq!(
            String::from("zqM="),
            encode("Σ")
        );

        assert_eq!(
            String::from("zqPOow=="),
            encode("ΣΣ")
        );
    }
    
    #[test]
    fn test_longer_encoding() {
        assert_eq!(String::from("VVBEQ3VGNW0yRlBQUGdaeXh6YVdGWXYxMG5UVWYyM1M1OFU1a0RDeWRYRjRyUDdBN1dFUGdROUt3akc1WDA2VGo2MXQ3Uk5HNFRDWEpYR0JrVm1pSGhack91RmNnVDR4TDRBc0R4d3dtd2N2bHlKU1RqUXZkem43MHd1SXhYZVVBbnI5bUw4TjhyalBnWG5CakRNZWNhZUUxOElWbXcyZ3F1MjFHaDR1N0ZZeHcwSG1SQUtRcFFQWFNGNzVka2Jjb3NVT1AzT1VNVjd6ZFNxYTVhckRDM2pUT3F4N0llaVc1TzNIa2hqblo0dVdJeElVUnZOaVppbDZZNXNCUTJUSkpldldQN1dIaVdOWEprYU92WU9qMjg0Z0NHeXptYlZJN3ZvSnM0eVdLZUdNdWhSYk9Sczg5aEk2dmlVY25ZNmoyVHE5Q2UyYlNLMlZtRkxQaVd0bzdFQ0IzYzBDeFZsbUtaVGtyOHRtMUpUT2hadVltOXlVbkQxdHRzc3ZSemcwZVhFMVdjREZjUmZ1dEZINENHeEdqYU9sUTJxaktkTElaQ0ZBejcxNnJMa096SkdHSDRrRXVza2UyVDhPT3R4Y2xWZ3JHcktNMTh5YXJiS0xBalBYYkJudWlHVTRUUFMxUDkzZHJyRExIbklJTlpDeTExdXJyNHFEMWNhODkxZUlzZ1FFQ3F6d0piY1NiTFlFMTVZNzhqS1VtdmYwQ2RpNW1ZTTM0MlVKU3QyVjZQSHRybkFmZXVvT1o2ZGtKNjlrV014ck10ZldES0VzZ01jNHlLU1hudEpIZHhpbFpYVFNwZk5Lb0hLWk9IbWRKSEFuRVlVWUNnTk9waExiQ2NRNHNKazQ3aWpGTWZRWklPV1Q2Y1NDdHI1VzlHS0dBb1BPb2dPQnczMmFDektJbjR4dmhxREdPZ3ZMcTUwR3Y4QzdKYk1ZODdEMHdtTGNhS2FON1JCbFU1bGRUWDFqdmlGNGZmV21IQ2Y1VkhMRzJXbjZWS0xyMXBqVlFwdExiZlZITGxoTjd1WTR1WkdjSmRuNm1ZTmtmTWhPSWFRWHJ2d0tTaFU1RkZucTlzZHI5bnBCYUNaSU4xNEM0bFpId09KUXoySzl6OHJtSkRZSlQ4R0RLVFNLeDBVNmJaR0ZBM3VzN0dxelBoc0VKUXhpWktIbVRlekpQVTRPUlAwYm5TUzgwd2FaSlR4V3Zic0hUeXEyTjQ0RjZYM0lhcTMwaHdpVGtBTDdoUWRoWVBYUGEybXl2Z0lvNE16UWtwUFBPTms2bzlOM3lCUjA1TVk0VXZPbmdOZ0R2RUZXZ2xaMlpwZmpoQ1lhZnFxdVRv"), encode("UPDCuF5m2FPPPgZyxzaWFYv10nTUf23S58U5kDCydXF4rP7A7WEPgQ9KwjG5X06Tj61t7RNG4TCXJXGBkVmiHhZrOuFcgT4xL4AsDxwwmwcvlyJSTjQvdzn70wuIxXeUAnr9mL8N8rjPgXnBjDMecaeE18IVmw2gqu21Gh4u7FYxw0HmRAKQpQPXSF75dkbcosUOP3OUMV7zdSqa5arDC3jTOqx7IeiW5O3HkhjnZ4uWIxIURvNiZil6Y5sBQ2TJJevWP7WHiWNXJkaOvYOj284gCGyzmbVI7voJs4yWKeGMuhRbORs89hI6viUcnY6j2Tq9Ce2bSK2VmFLPiWto7ECB3c0CxVlmKZTkr8tm1JTOhZuYm9yUnD1ttssvRzg0eXE1WcDFcRfutFH4CGxGjaOlQ2qjKdLIZCFAz716rLkOzJGGH4kEuske2T8OOtxclVgrGrKM18yarbKLAjPXbBnuiGU4TPS1P93drrDLHnIINZCy11urr4qD1ca891eIsgQECqzwJbcSbLYE15Y78jKUmvf0Cdi5mYM342UJSt2V6PHtrnAfeuoOZ6dkJ69kWMxrMtfWDKEsgMc4yKSXntJHdxilZXTSpfNKoHKZOHmdJHAnEYUYCgNOphLbCcQ4sJk47ijFMfQZIOWT6cSCtr5W9GKGAoPOogOBw32aCzKIn4xvhqDGOgvLq50Gv8C7JbMY87D0wmLcaKaN7RBlU5ldTX1jviF4ffWmHCf5VHLG2Wn6VKLr1pjVQptLbfVHLlhN7uY4uZGcJdn6mYNkfMhOIaQXrvwKShU5FFnq9sdr9npBaCZIN14C4lZHwOJQz2K9z8rmJDYJT8GDKTSKx0U6bZGFA3us7GqzPhsEJQxiZKHmTezJPU4ORP0bnSS80waZJTxWvbsHTyq2N44F6X3Iaq30hwiTkAL7hQdhYPXPa2myvgIo4MzQkpPPONk6o9N3yBR05MY4UvOngNgDvEFWglZ2ZpfjhCYafqquTo"))
    }

    #[test]
    fn test_encode_1_char() {
        assert_eq!(String::from("YQ=="), encode("a"))
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            decode("aGVsbG90aGVyZXVpamtsbw=="),
            String::from("hellothereuijklo")
        );

        assert_eq!(decode("c3VybmFqYW4="), String::from("surnajan"));
        assert_eq!(decode("Ym90dGwxMjEy"), String::from("bottl1212"));
        assert_eq!(decode("QXNwaGFsdEA4OTA="), String::from("Asphalt@890"));
        assert_eq!(
            decode("KHNhUy5zYXNkYXcvd2Fpb25za2F3K2FzYXM9c2FkYXdheiNhc2R3"),
            String::from("(saS.sasdaw/waionskaw+asas=sadawaz#asdw")
        );
        assert_eq!(
            decode("ODkyM2prbXdlcDItPXBva21hczEuLC9hczthbHNtYWQ="),
            String::from("8923jkmwep2-=pokmas1.,/as;alsmad")
        );
        assert_eq!(
            decode("XHBvcG9rZXdubXBjITJrc21kU2xta3Ns"),
            String::from("\\popokewnmpc!2ksmdSlmksl")
        );

        assert_eq!(
            String::from("Σ"),
            decode("zqM=")
        );

        assert_eq!(
            String::from("ΣΣ"),
            decode("zqPOow==")
        );
    }
    
     #[test]
    fn test_longer_decoding() {
        assert_eq!(
            decode("VVBEQ3VGNW0yRlBQUGdaeXh6YVdGWXYxMG5UVWYyM1M1OFU1a0RDeWRYRjRyUDdBN1dFUGdROUt3akc1WDA2VGo2MXQ3Uk5HNFRDWEpYR0JrVm1pSGhack91RmNnVDR4TDRBc0R4d3dtd2N2bHlKU1RqUXZkem43MHd1SXhYZVVBbnI5bUw4TjhyalBnWG5CakRNZWNhZUUxOElWbXcyZ3F1MjFHaDR1N0ZZeHcwSG1SQUtRcFFQWFNGNzVka2Jjb3NVT1AzT1VNVjd6ZFNxYTVhckRDM2pUT3F4N0llaVc1TzNIa2hqblo0dVdJeElVUnZOaVppbDZZNXNCUTJUSkpldldQN1dIaVdOWEprYU92WU9qMjg0Z0NHeXptYlZJN3ZvSnM0eVdLZUdNdWhSYk9Sczg5aEk2dmlVY25ZNmoyVHE5Q2UyYlNLMlZtRkxQaVd0bzdFQ0IzYzBDeFZsbUtaVGtyOHRtMUpUT2hadVltOXlVbkQxdHRzc3ZSemcwZVhFMVdjREZjUmZ1dEZINENHeEdqYU9sUTJxaktkTElaQ0ZBejcxNnJMa096SkdHSDRrRXVza2UyVDhPT3R4Y2xWZ3JHcktNMTh5YXJiS0xBalBYYkJudWlHVTRUUFMxUDkzZHJyRExIbklJTlpDeTExdXJyNHFEMWNhODkxZUlzZ1FFQ3F6d0piY1NiTFlFMTVZNzhqS1VtdmYwQ2RpNW1ZTTM0MlVKU3QyVjZQSHRybkFmZXVvT1o2ZGtKNjlrV014ck10ZldES0VzZ01jNHlLU1hudEpIZHhpbFpYVFNwZk5Lb0hLWk9IbWRKSEFuRVlVWUNnTk9waExiQ2NRNHNKazQ3aWpGTWZRWklPV1Q2Y1NDdHI1VzlHS0dBb1BPb2dPQnczMmFDektJbjR4dmhxREdPZ3ZMcTUwR3Y4QzdKYk1ZODdEMHdtTGNhS2FON1JCbFU1bGRUWDFqdmlGNGZmV21IQ2Y1VkhMRzJXbjZWS0xyMXBqVlFwdExiZlZITGxoTjd1WTR1WkdjSmRuNm1ZTmtmTWhPSWFRWHJ2d0tTaFU1RkZucTlzZHI5bnBCYUNaSU4xNEM0bFpId09KUXoySzl6OHJtSkRZSlQ4R0RLVFNLeDBVNmJaR0ZBM3VzN0dxelBoc0VKUXhpWktIbVRlekpQVTRPUlAwYm5TUzgwd2FaSlR4V3Zic0hUeXEyTjQ0RjZYM0lhcTMwaHdpVGtBTDdoUWRoWVBYUGEybXl2Z0lvNE16UWtwUFBPTms2bzlOM3lCUjA1TVk0VXZPbmdOZ0R2RUZXZ2xaMlpwZmpoQ1lhZnFxdVRv"),
            String::from("UPDCuF5m2FPPPgZyxzaWFYv10nTUf23S58U5kDCydXF4rP7A7WEPgQ9KwjG5X06Tj61t7RNG4TCXJXGBkVmiHhZrOuFcgT4xL4AsDxwwmwcvlyJSTjQvdzn70wuIxXeUAnr9mL8N8rjPgXnBjDMecaeE18IVmw2gqu21Gh4u7FYxw0HmRAKQpQPXSF75dkbcosUOP3OUMV7zdSqa5arDC3jTOqx7IeiW5O3HkhjnZ4uWIxIURvNiZil6Y5sBQ2TJJevWP7WHiWNXJkaOvYOj284gCGyzmbVI7voJs4yWKeGMuhRbORs89hI6viUcnY6j2Tq9Ce2bSK2VmFLPiWto7ECB3c0CxVlmKZTkr8tm1JTOhZuYm9yUnD1ttssvRzg0eXE1WcDFcRfutFH4CGxGjaOlQ2qjKdLIZCFAz716rLkOzJGGH4kEuske2T8OOtxclVgrGrKM18yarbKLAjPXbBnuiGU4TPS1P93drrDLHnIINZCy11urr4qD1ca891eIsgQECqzwJbcSbLYE15Y78jKUmvf0Cdi5mYM342UJSt2V6PHtrnAfeuoOZ6dkJ69kWMxrMtfWDKEsgMc4yKSXntJHdxilZXTSpfNKoHKZOHmdJHAnEYUYCgNOphLbCcQ4sJk47ijFMfQZIOWT6cSCtr5W9GKGAoPOogOBw32aCzKIn4xvhqDGOgvLq50Gv8C7JbMY87D0wmLcaKaN7RBlU5ldTX1jviF4ffWmHCf5VHLG2Wn6VKLr1pjVQptLbfVHLlhN7uY4uZGcJdn6mYNkfMhOIaQXrvwKShU5FFnq9sdr9npBaCZIN14C4lZHwOJQz2K9z8rmJDYJT8GDKTSKx0U6bZGFA3us7GqzPhsEJQxiZKHmTezJPU4ORP0bnSS80waZJTxWvbsHTyq2N44F6X3Iaq30hwiTkAL7hQdhYPXPa2myvgIo4MzQkpPPONk6o9N3yBR05MY4UvOngNgDvEFWglZ2ZpfjhCYafqquTo")
        );
    }
}