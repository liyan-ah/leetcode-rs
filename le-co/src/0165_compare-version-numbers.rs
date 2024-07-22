impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let ver1: Vec<&str> = version1.split(".").collect();
        let ver2: Vec<&str> = version2.split(".").collect();
        let (mut iter1, mut iter2) = (ver1.iter(), ver2.iter());
        let (mut v1, mut v2) = (iter1.next(), iter2.next());
        loop {
            if v1 == None || v2 == None {
                break;
            }
            let str1 = v1.unwrap();
            let str2 = v2.unwrap();
            let code1 = str1.parse::<i32>().unwrap();
            let code2 = str2.parse::<i32>().unwrap();
            if code1 < code2 {
                return -1;
            }
            if code1 > code2 {
                return 1;
            }
            v1 = iter1.next();
            v2 = iter2.next();
        }
        if v1 == None {
            loop {
                if v2 == None{
                    break;
                }
                let str2 = v2.unwrap();
                let code2 = str2.parse::<i32>().unwrap();
                if code2 != 0 {
                    return -1;
                }
                v2 = iter2.next();
            }
            return 0;
        }
        if v2 == None{
            loop{
                if v1 == None{
                    break;
                }
                let str1 = v1.unwrap();
                let code1 = str1.parse::<i32>().unwrap();
                if code1 != 0{
                    return 1;
                }
                v1 = iter1.next();
            }
            return 0;
        }
        return 0;
    }
}
